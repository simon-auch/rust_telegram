use crate::http_stream::{HttpMsg, HttpStream};
use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::sync;
use async_tls::TlsAcceptor;
use core::result::Result;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use futures::task::{LocalSpawn, LocalSpawnExt};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::{Path,PathBuf};
use std::sync::Arc;

/// Load the passed certificates file
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
}

/// Load the passed keys file
fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
}

/// Configure the server using rusttls
/// See https://docs.rs/rustls/0.16.0/rustls/struct.ServerConfig.html for details
///
/// A TLS server needs a certificate and a fitting private key
fn load_config(cert: &Path, key: &Path) -> io::Result<ServerConfig> {
    let certs = load_certs(&cert)?;
    let mut keys = load_keys(&key)?;
    println!("Certs loaded: {}", certs.len());
    println!("Keys loaded: {}", keys.len());

    // we don't use client authentication
    let mut config = ServerConfig::new(NoClientAuth::new());
    config
        // set this server to use one cert together with the loaded private key
        .set_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    Ok(config)
}

///Config for the receiver
pub struct Config {
    addr: SocketAddr,
    certificate: PathBuf,
    certificate_key: PathBuf,
    webhook_path: String,
}

impl Config {
    pub fn new(
        addr: SocketAddr,
        certificate: PathBuf,
        certificate_key: PathBuf,
        webhook_path: String,
    ) -> Self {
        Config {
            addr,
            certificate,
            certificate_key,
            webhook_path,
        }
    }
}

impl Config {
    fn tls_acceptor(&self) -> TlsAcceptor {
        let config = load_config(
            Path::new(&self.certificate),
            Path::new(&self.certificate_key),
        )
        .unwrap();

        // We create one TLSAcceptor around a shared configuration.
        // Cloning the acceptor will not clone the configuration.
        TlsAcceptor::from(Arc::new(config))
    }
}

pub type SendItem = (HttpMsg, sync::Sender<HttpMsg>);

///Receiver
/// accepts tcp connections on the speciefied port
/// for each connection creates a new task and hands them of to the TcpStreamHandler
pub struct TelegramReceiver {
    config: Config,
    tls_acceptor: TlsAcceptor,
    stop: sync::Receiver<()>,
    //gets passed to the TcpStreamHandler
    output: sync::Sender<SendItem>,
}

///TcpStreamHandler
/// performs the tls handshake on the stream and hands it over to a HttpStreamHandler
struct TcpStreamHandler {
    tcp_stream: TcpStream,
    tls_acceptor: TlsAcceptor,
    stop: sync::Receiver<()>,
    output: sync::Sender<SendItem>,
}

///HttpStreamHandler
/// reads from the provided HttpStream in a loop and dispatches the read HttpMsg and the HttpStream into output
pub struct HttpStreamHandler {
    http_stream: HttpStream<async_tls::server::TlsStream<TcpStream>>,
    stop: sync::Receiver<()>,
    output: sync::Sender<SendItem>,
}

impl TelegramReceiver {
    pub fn new(config: Config) -> (Self, sync::Sender<()>, sync::Receiver<SendItem>) {
        let (stop_send, stop_recv) = sync::channel(1);
        let (http_send, http_recv) = sync::channel(10);
        let tls_acceptor = config.tls_acceptor();
        (
            TelegramReceiver {
                config: config,
                tls_acceptor: tls_acceptor,
                stop: stop_recv,
                output: http_send,
            },
            stop_send,
            http_recv,
        )
    }
    pub fn get_webhook_uri(&self) -> String {
        self.config.webhook_path.clone()
    }
    pub async fn run<S>(self, executor: S)
    where
        S: LocalSpawn,
    {
        let mut stop_fused = self.stop.clone().fuse();
        let tcp_listener = TcpListener::bind(&self.config.addr).await.unwrap();
        let mut incoming = tcp_listener.incoming().fuse();

        loop {
            select! {
                _ = stop_fused.next() => {
                    println!("Receiver: received stop");
                    return
                },
                result = incoming.next() => {
                    println!("Receiver: new incomming connection");
                    match result.unwrap() {
                        Result::Ok(tcp_stream) => {
                            let tcp_stream_handler = TcpStreamHandler{
                                tcp_stream: tcp_stream,
                                tls_acceptor: self.tls_acceptor.clone(),
                                stop: self.stop.clone(),
                                output: self.output.clone(),
                            };
                            executor.spawn_local(
                                tcp_stream_handler.handle()
                            ).unwrap();
                        },
                        Result::Err(err) => {},
                    }
                },
            };
        }
    }
}

impl TcpStreamHandler {
    async fn handle(self) {
        let mut stop_fused = self.stop.clone().fuse();
        select! {
            _ = stop_fused.next() => {
                println!("HttpStreamHandler: received stop");
                return
            },
            result = self.tls_acceptor.accept(self.tcp_stream).fuse() => {
                if result.is_err() {
                    let err = result.err().unwrap();
                    println!("Error during handshake:");
                    println!("{}", err);
                    return;
                }
                let stream = result.ok().unwrap();
                let http_stream_handler = HttpStreamHandler {
                    http_stream: HttpStream::new(stream),
                    stop: self.stop,
                    output: self.output,
                };
                http_stream_handler.handle().await;
            },
        }
    }
}

impl HttpStreamHandler {
    pub fn new(
        http_stream: HttpStream<async_tls::server::TlsStream<TcpStream>>,
        stop: sync::Receiver<()>,
        output: sync::Sender<SendItem>,
    ) -> Self {
        HttpStreamHandler {
            http_stream,
            stop,
            output,
        }
    }
    pub async fn handle(mut self) {
        let mut stop_fused = self.stop.clone().fuse();
        loop {
            select! {
                _ = stop_fused.next() => {
                    println!("HttpStreamHandler: received stop");
                    return
                },
                http_msg = self.http_stream.read().fuse() => {
                    if http_msg.is_err() {
                        println!("HttpStreamHandler: IO error");
                        //io error or parsing error, exit
                        return;
                    }
                    let http_msg = http_msg.ok().unwrap();
                    //println!("{:?}", http_msg);
                    println!(
                        "{}",
                        String::from_utf8(http_msg.get_body().clone()).unwrap()
                    );
                    let (response_send, response_recv) = sync::channel(1);
                    self.output.send((http_msg, response_send)).await;
                    //wait for respond to send
                    let response = response_recv.recv().await;
                    //or create one if none provided
                    let response = match response {
                        Some(resp) => resp,
                        None => HttpMsg::new_respone(200),
                    };
                    self.http_stream.write(response).await;
                    //dunno how we could really respond here to an error
                },
            }
        }
    }
}
