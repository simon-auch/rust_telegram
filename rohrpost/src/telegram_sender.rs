use crate::http_stream;
use crate::http_stream::{HttpMsg, HttpStream};
use crate::telegram_methods;
use crate::telegram_methods::TelegramMethod;
use crate::telegram_receiver::TelegramReceiver;
use async_std::net::TcpStream;
use async_tls::TlsConnector;
use futures::{AsyncRead, AsyncWrite};
use http::request::Request;
use serde::ser::Serialize;
use serde_json;

const TELEGRAM_API_BASE: &'static str = "https://api.telegram.org/bot";

pub struct Config {
    token: String,
}

impl Config {
    pub fn new(token: String) -> Self {
        Config { token: token }
    }
}

pub struct TelegramSender {
    config: Config,
}

impl TelegramSender {
    pub fn new(config: Config) -> Self {
        TelegramSender { config: config }
    }
    fn uri<M>(&self) -> String
    where
        M: TelegramMethod,
    {
        let mut temp = String::new();
        temp += TELEGRAM_API_BASE;
        temp += &self.config.token;
        temp += "/";
        temp += M::method_name;
        temp
    }
    pub async fn call_on_http_stream<M, S>(
        &self,
        method: &M,
        http_stream: &mut HttpStream<S>,
    ) -> std::io::Result<()>
    where
        M: TelegramMethod + Serialize,
        S: AsyncWrite + AsyncRead + std::marker::Unpin,
    {
        let uri = self.uri::<M>();

        //construct our message
        let body = serde_json::to_vec(method).unwrap();
        let req = Request::builder()
            .method("POST")
            .uri(&uri)
            .header("host", "api.telegram.org")
            .header("content-length", body.len())
            .header("content-type", "application/json")
            .body(body)
            .unwrap();
        let http_msg = HttpMsg::Request(req);

        //send
        http_stream.write(http_msg).await?;
        Ok(())
    }
    ///Calls the telegram method. Creates a new tls stream and returns the response.
    pub async fn call<M>(&self, method: &M) -> std::io::Result<HttpMsg>
    where
        M: TelegramMethod + Serialize,
    {
        let tcp_stream = TcpStream::connect("api.telegram.org:443").await?;
        let connector = TlsConnector::default();
        let handshake = connector.connect("api.telegram.org", tcp_stream)?;
        let tls_stream = handshake.await?;
        let mut http_stream = HttpStream::new(tls_stream);
        //send
        self.call_on_http_stream(method, &mut http_stream).await?;
        //receive result
        let http_msg = http_stream.read().await;
        if http_msg.is_err() {
            //io error or parsing error
            match http_msg.err().unwrap() {
                http_stream::Error::IO(err) => return Err(err),
                http_stream::Error::Parse(()) => {
                    println!("Could not parse");
                    return Err(std::io::ErrorKind::InvalidData.into());
                }
            }
        }
        let http_msg = http_msg.ok().unwrap();
        Ok(http_msg)
    }

    pub async fn register_web_hook(&self, receiver: &TelegramReceiver) -> std::io::Result<()> {
        let method = telegram_methods::setWebhookBuilder::default()
            .url(receiver.get_webhook_uri())
            .build()
            .unwrap();
        let http_msg = self.call(&method).await?;
        assert_eq!(http_msg.get_response().status().as_u16(), 200);
        Ok(())
    }
}
