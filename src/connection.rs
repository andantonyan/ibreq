use crate::config::ControllerConfig;
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

pub fn create_stream(conf: &ControllerConfig) -> Box<dyn Connection> {
  if conf.ssl {
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(conf.get_addr()).unwrap();

    Box::new(connector.connect(&conf.host, stream).unwrap())
  } else {
    let stream = TcpStream::connect(conf.get_addr()).unwrap();

    Box::new(stream)
  }
}
