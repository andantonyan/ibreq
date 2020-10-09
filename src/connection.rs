use crate::Result;
use native_tls::{TlsConnector, TlsStream};
use std::{io::Read, io::Result as IResult, io::Write, net::TcpStream};
pub trait Connection {
  fn w(&mut self, buf: &[u8]) -> IResult<usize>;
  fn r(&mut self, buf: &mut String) -> IResult<usize>;
}

impl Connection for TlsStream<TcpStream> {
  fn w(&mut self, buf: &[u8]) -> IResult<usize> {
    self.write(buf)
  }

  fn r(&mut self, buf: &mut String) -> IResult<usize> {
    self.read_to_string(buf)
  }
}

impl Connection for TcpStream {
  fn w(&mut self, buf: &[u8]) -> IResult<usize> {
    self.write(buf)
  }

  fn r(&mut self, buf: &mut String) -> IResult<usize> {
    self.read_to_string(buf)
  }
}

pub fn create_stream(ssl: bool, host: &str, port: u16) -> Result<Box<dyn Connection>> {
  let addr = format!("{}:{}", host, port);
  if ssl {
    let connector = TlsConnector::new()?;
    let stream = TcpStream::connect(addr)?;

    Ok(Box::new(connector.connect(host, stream)?))
  } else {
    let stream = TcpStream::connect(addr)?;

    Ok(Box::new(stream))
  }
}
