use crate::http_stream::{HttpMsg, HttpStream};
use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::sync;
use async_tls::TlsAcceptor;
use core::result::Result;
use futures::channel::oneshot;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use futures::task::{LocalSpawn, LocalSpawnExt};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::Path;
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

pub struct Config {
    addr: SocketAddr,
    certificate: String,
    certificate_key: String,
    webhook_path: String,
}

impl Config {
    pub fn new(
        addr: SocketAddr,
        certificate: String,
        certificate_key: String,
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

pub type SendItem = (HttpStream<async_tls::server::TlsStream<TcpStream>>, HttpMsg);

pub struct Receiver {
    config: Config,

    tls_acceptor: TlsAcceptor,

    stop: oneshot::Receiver<()>,
    output: sync::Sender<SendItem>,
}

impl Receiver {
    pub fn new(config: Config) -> (Self, oneshot::Sender<()>, sync::Receiver<SendItem>) {
        let (stop_send, stop_recv) = oneshot::channel();
        let (output_send, output_recv) = sync::channel(10);

        let tls_acceptor = config.tls_acceptor();

        let receiver = Receiver {
            config: config,
            tls_acceptor: tls_acceptor,
            stop: stop_recv,
            output: output_send,
        };
        (receiver, stop_send, output_recv)
    }
    pub fn get_webhook_uri(&self) -> String {
        self.config.webhook_path.clone()
    }
    pub async fn run<S>(self, executor: S)
    where
        S: LocalSpawn,
    {
        let mut stop = self.stop.fuse();
        let tcp_listener = TcpListener::bind(&self.config.addr).await.unwrap();
        let mut incoming = tcp_listener.incoming().fuse();

        loop {
            select! {
                _ = stop => {
                    println!("Received stop");
                    return
                },
                result = incoming.next() => {
                    match result.unwrap() {
                        Result::Ok(tcp_stream) => {
                            executor.spawn_local(
                                Self::handle_tcp_stream(tcp_stream, self.tls_acceptor.clone(), self.output.clone())
                            ).unwrap();
                        },
                        Result::Err(err) => {},
                    }
                },
            };
        }
    }
    async fn handle_tcp_stream(
        tcp_stream: TcpStream,
        tls_acceptor: TlsAcceptor,
        output: sync::Sender<SendItem>,
    ) {
        let result = tls_acceptor.accept(tcp_stream).await;
        if result.is_err() {
            let err = result.err().unwrap();
            println!("Error during handshake:");
            println!("{}", err);
            return;
        }
        let stream = result.ok().unwrap();
        let mut http_stream = HttpStream::new(stream);
        let http_msg = http_stream.read().await;
        if http_msg.is_err() {
            //io error or parsing error, exit
            return;
        }
        let http_msg = http_msg.ok().unwrap();
        //println!("{:?}", http_msg);
        println!(
            "{}",
            String::from_utf8(http_msg.get_body().clone()).unwrap()
        );
        output.send((http_stream, http_msg)).await;
    }
}
