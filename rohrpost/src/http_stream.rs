use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use http;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use http::request::Request;
use http::response::Response;
use http::Method;
use http::StatusCode;
use http::Uri;
use http::Version;
use std::convert::TryFrom;
use std::io;

const CR: u8 = 0x0D;
const LF: u8 = 0x0A;
const SPACE: u8 = 0x20u8;
const COLON: u8 = 0x3Au8;

pub struct HttpStream<S> {
    stream: S,
}

impl<S> HttpStream<S> {
    pub fn new(stream: S) -> Self {
        HttpStream { stream: stream }
    }
}

impl<S> HttpStream<S>
where
    S: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    async fn read_until_crlf(&mut self) -> io::Result<Vec<u8>> {
        let crlf = [CR, LF];
        let mut temp = [0u8, 0];
        //println!("start reading until crlf");
        self.stream.read_exact(&mut temp).await?;
        let mut buf = Vec::new();
        loop {
            if crlf == temp {
                break;
            }
            buf.push(temp[0]);
            temp[0] = temp[1];
            //read the next byte
            self.stream.read_exact(&mut temp[1..]).await?;
        }
        //println!("end reading until crlf");
        Ok(buf)
    }
    fn parse_request_line(line: &[u8]) -> std::result::Result<(Method, Uri, Version), ()> {
        let mut split_1 = None;
        let mut split_2 = None;
        for i in 0..line.len() {
            if line[i] == SPACE {
                if split_1.is_none() {
                    split_1 = Some(i);
                } else {
                    split_2 = Some(i);
                    break;
                }
            }
        }
        if split_2.is_none() {
            println!("Could not find two spaces in request line");
            return Err(());
        }
        let split_1 = split_1.unwrap();
        let split_2 = split_2.unwrap();
        let method = Method::from_bytes(&line[..split_1]);
        if method.is_err() {
            //println!("Could not parse method");
            return Err(());
        }
        let method = method.unwrap();

        let uri = Uri::try_from(&line[split_1 + 1..split_2]);
        if uri.is_err() {
            println!("Could not parse uri");
            return Err(());
        }
        let uri = uri.unwrap();

        //TODO correctly parse the version
        Ok((method, uri, Version::HTTP_11))
    }
    fn parse_status_line(line: &[u8]) -> std::result::Result<(Version, StatusCode), ()> {
        let mut split_1 = None;
        let mut split_2 = None;
        for i in 0..line.len() {
            if line[i] == SPACE {
                if split_1.is_none() {
                    split_1 = Some(i);
                } else {
                    split_2 = Some(i);
                    break;
                }
            }
        }
        if split_1.is_none() {
            //println!("Could not find first spaces in request line");
            return Err(());
        }
        let split_1 = split_1.unwrap();

        let status_code = StatusCode::from_bytes(match split_2 {
            Some(split_2) => &line[split_1 + 1..split_2],
            None => &line[split_1 + 1..],
        });
        if status_code.is_err() {
            //println!("Could not parse status code");
            return Err(());
        }
        let status_code = status_code.unwrap();

        //TODO correctly parse the version
        Ok((Version::HTTP_11, status_code))
    }
    fn parse_header_line(line: &[u8]) -> std::result::Result<(HeaderName, HeaderValue), ()> {
        let mut split = None;
        for i in 0..line.len() {
            if line[i] == COLON {
                split = Some(i);
                break;
            }
        }
        if split.is_none() {
            //println!("Could not find colon in header line");
            return Err(());
        }
        let split = split.unwrap();
        let header_name = HeaderName::from_bytes(&line[..split]);
        if header_name.is_err() {
            //println!("Could not parse header name");
            return Err(());
        }
        let header_name = header_name.ok().unwrap();
        let header_value = HeaderValue::from_bytes(&line[split + 1..]);
        if header_value.is_err() {
            //println!("Could not parse header value");
            return Err(());
        }
        let header_value = header_value.ok().unwrap();
        Ok((header_name, header_value))
    }
    pub async fn read(&mut self) -> std::result::Result<HttpMsg, Error> {
        //first read the request/status line
        let request_status_line = self.read_until_crlf().await;
        if request_status_line.is_err() {
            return Err(Error::IO(request_status_line.err().unwrap()));
        }
        let request_status_line = request_status_line.ok().unwrap();
        let request = Self::parse_request_line(&request_status_line);
        let status = Self::parse_status_line(&request_status_line);
        if request.is_err() && status.is_err() {
            println!("could not parse first line as either request or status.");
            return Err(Error::Parse(()));
        }
        if request.is_ok() && status.is_ok() {
            println!("could parse first line as either request or status");
            return Err(Error::Parse(()));
        }

        //read the header lines (until empty line with only crlf)
        let mut header_map = HeaderMap::new();
        loop {
            //read line
            let line = self.read_until_crlf().await;
            if line.is_err() {
                return Err(Error::IO(line.err().unwrap()));
            }
            let line = line.ok().unwrap();
            //check if its empty
            if line.len() == 0 {
                break;
            }
            //parse line
            let result = Self::parse_header_line(&line);
            if result.is_err() {
                return Err(Error::Parse(result.err().unwrap()));
            }
            let (header_name, header_value) = result.ok().unwrap();
            header_map.insert(header_name, header_value);
        }
        //println!("Header Map: {:?}", header_map);
        let body = if header_map.contains_key(http::header::CONTENT_LENGTH) {
            let content_length = header_map.get(http::header::CONTENT_LENGTH).unwrap();
            let content_length = content_length.to_str();
            if content_length.is_err() {
                println!("could not convert content length to string");
                return Err(Error::Parse(()));
            }
            let content_length = content_length.unwrap();
            let content_length = String::from(content_length.trim());
            let content_length = content_length.parse::<usize>();
            if content_length.is_err() {
                println!("could not convert content length to usize");
                return Err(Error::Parse(()));
            }
            let content_length = content_length.unwrap();
            let mut buf: Vec<u8> = Vec::with_capacity(content_length);
            for _ in 0..content_length {
                buf.push(0);
            }
            //println!("start read body");
            let ret = self.stream.read_exact(&mut buf[..]).await;
            //println!("end read body");
            if ret.is_err() {
                //println!("could not read body");
                return Err(Error::IO(ret.err().unwrap()));
            }
            buf
        } else {
            Vec::new()
        };
        if request.is_ok() {
            let (method, uri, version) = request.unwrap();
            let request = Request::builder()
                .method(method)
                .uri(uri)
                .version(version)
                .body(body)
                .unwrap();
            Ok(HttpMsg::Request(request))
        } else {
            let (version, status_code) = status.unwrap();
            let response = Response::builder()
                .version(version)
                .status(status_code)
                .body(body)
                .unwrap();
            Ok(HttpMsg::Response(response))
        }
    }
    pub async fn write(&mut self, msg: HttpMsg) -> io::Result<()> {
        let mut send: Vec<u8> = Vec::new();
        match msg {
            HttpMsg::Request(req) => {
                let (parts, body) = req.into_parts();
                for &byte in parts.method.as_str().as_bytes() {
                    send.push(byte);
                }
                send.push(SPACE);
                let temp = format!("{}", parts.uri);
                for &byte in temp.as_bytes() {
                    send.push(byte);
                }
                send.push(SPACE);
                let temp = format!("{:?}", parts.version);
                for &byte in temp.as_bytes() {
                    send.push(byte);
                }
                send.push(CR);
                send.push(LF);
                for key in parts.headers.keys() {
                    let value = parts.headers.get(key).unwrap();
                    for &byte in key.as_str().as_bytes() {
                        send.push(byte);
                    }
                    send.push(COLON);
                    send.push(SPACE);
                    for &byte in value.as_bytes() {
                        send.push(byte);
                    }
                    send.push(CR);
                    send.push(LF);
                }
                send.push(CR);
                send.push(LF);
                for &byte in body.iter() {
                    send.push(byte);
                }
            }
            HttpMsg::Response(resp) => {
                let (parts, body) = resp.into_parts();
                let temp = format!("{:?}", parts.version);
                for &byte in temp.as_bytes() {
                    send.push(byte);
                }
                send.push(SPACE);
                for &byte in parts.status.as_str().as_bytes() {
                    send.push(byte);
                }
                send.push(CR);
                send.push(LF);
                for key in parts.headers.keys() {
                    let value = parts.headers.get(key).unwrap();
                    for &byte in key.as_str().as_bytes() {
                        send.push(byte);
                    }
                    send.push(COLON);
                    send.push(SPACE);
                    for &byte in value.as_bytes() {
                        send.push(byte);
                    }
                    send.push(CR);
                    send.push(LF);
                }
                send.push(CR);
                send.push(LF);
                for &byte in body.iter() {
                    send.push(byte);
                }
            }
        }
        println!("{}", String::from_utf8(send.clone()).unwrap());
        self.stream.write_all(&send[..]).await?;
        Ok(())
    }
}

pub enum Error {
    IO(io::Error),
    Parse(()),
}

#[derive(Debug)]
pub enum HttpMsg {
    Request(http::request::Request<Vec<u8>>),
    Response(http::response::Response<Vec<u8>>),
}

impl HttpMsg {
    pub fn get_body(&self) -> &Vec<u8> {
        match self {
            HttpMsg::Request(req) => req.body(),
            HttpMsg::Response(resp) => resp.body(),
        }
    }
    pub fn get_request(&self) -> &http::request::Request<Vec<u8>> {
        match self {
            HttpMsg::Request(req) => req,
            HttpMsg::Response(_resp) => panic!(),
        }
    }
    pub fn get_response(&self) -> &http::response::Response<Vec<u8>> {
        match self {
            HttpMsg::Request(_req) => panic!(),
            HttpMsg::Response(resp) => resp,
        }
    }
    pub fn new_respone(status: u16) -> Self {
        let response = http::Response::builder()
            .version(http::version::Version::HTTP_11)
            .status(http::status::StatusCode::from_u16(status).unwrap())
            .header("host", "api.telegram.org")
            .body(Vec::new())
            .unwrap();
        HttpMsg::Response(response)
    }
}
