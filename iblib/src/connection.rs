use crate::util::Result;
use native_tls::{TlsConnector, TlsStream};
use std::{
  fmt::Debug,
  io::Result as IResult,
  io::Write,
  io::{BufRead, BufReader, Read},
  net::TcpStream,
};
pub trait Connection: Read + Write + Debug {
  fn write_buf(&mut self, buf: &[u8]) -> IResult<usize>;
  fn read_buf(&mut self, buf: &mut [u8]) -> IResult<usize>;

  fn get_res(&mut self) -> IResult<String> {
    let mut reader = BufReader::new(self);
    // TODO: Read current current data in the TcpStream
    let received: Vec<u8> = reader.fill_buf()?.to_vec();

    // TODO: Do some processing or validation to make sure the whole line is present?
    // TODO: Mark the bytes read as consumed so the buffer will not return them in a subsequent read
    reader.consume(received.len());

    Ok(String::from_utf8(received).unwrap_or_default())
  }
}

impl Connection for TlsStream<TcpStream> {
  fn write_buf(&mut self, buf: &[u8]) -> IResult<usize> {
    self.write(buf)
  }

  fn read_buf(&mut self, buf: &mut [u8]) -> IResult<usize> {
    self.read(buf)
  }
}

impl Connection for TcpStream {
  fn write_buf(&mut self, buf: &[u8]) -> IResult<usize> {
    self.write(buf)
  }

  fn read_buf(&mut self, buf: &mut [u8]) -> IResult<usize> {
    self.read(buf)
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
