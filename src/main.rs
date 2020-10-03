use std::{error, io::Read, io::Write, thread, time::Duration, time::Instant, net::TcpStream};
use rand::{Rng, thread_rng};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

static MAX_BUFFER_SIZE: u16 = 1024; 
static SEPARATOR: &str = "\r\n\r\n";
static CONF_ADDR: &str = "localhost:3000";

#[derive(Debug, Clone)]
struct Config {
  headers: String,
  addr: String,
  content_length: u16,
  thread_count: u8,
  call_interval_in_ms: u64,
  config_fetch_interval_in_ms: u64,
  state: String,
}

#[derive(Debug, Clone)]
struct Response {
  headers: String,
  body: String
}

fn main() {
  loop {
    if let Ok(conf) = get_conf() {
      let start = Instant::now();
      let config_fetch_interval = Duration::from_millis(conf.config_fetch_interval_in_ms);
      let call_interval = Duration::from_millis(conf.call_interval_in_ms);

      if conf.state == "START" {
        'inner:loop {
          let mut threads = vec![];
  
          for i in 0..conf.thread_count {
            let conf = conf.clone();
            let thread = thread::spawn(move || match call(&conf) {
              Ok(res) => {
                println!("Thread {}, Response\n{:?}", i, res.headers);
                thread::sleep(call_interval);
              }
              Err(err) => {
                eprintln!("Thread {}, Unable to call {:?}.", i, err);
                thread::sleep(call_interval);
              },
            });
  
            threads.push(thread);
          }
  
          for thread in threads {
            let _ = thread.join();
          }
  
          if start.elapsed() > config_fetch_interval {
            break 'inner;
          }
        } 
      }

      thread::sleep(config_fetch_interval);
    } else {
      continue;
    }
  }
}

fn call(conf: &Config) -> Result<Response> {
  println!("Calling {}...", conf.addr);

  let mut stream = TcpStream::connect(conf.addr.to_owned())?;
  let mut rng = thread_rng();
  let body: Vec<u8> = vec![0; conf.content_length as usize].iter().map(|_| rng.gen::<u8>()).collect();
  let mut res = String::new();

  stream.write(conf.headers.as_bytes())?;
  stream.write("\n\n".as_bytes())?;

  if conf.content_length > MAX_BUFFER_SIZE {
    for chunk in body.chunks(MAX_BUFFER_SIZE as usize) {
      stream.write(&chunk)?;
    }
  } else {
    stream.write(&body)?;
  }
  
  stream.write(&[0; 1])?;
  stream.read_to_string(&mut res)?;

  println!("Done calling {}...", conf.addr);

  let res_chunks: Vec<&str> = res.split(SEPARATOR).collect();
  let res = Response {
    headers: res_chunks[0].to_string(),
    body: res_chunks[1].to_string()
  };

  return Ok(res);
}

fn get_conf() -> Result<Config> {
  println!("Fetching config...");
  
  let headers = String::from("GET / HTTP/1.1\nAccept: */*") + SEPARATOR;
  let mut res = String::new();
  let mut stream = TcpStream::connect(CONF_ADDR)?;
  
  stream.write(&headers.as_bytes())?;
  stream.read_to_string(&mut res)?;
  
  let res_chunks: Vec<&str> = res.split(SEPARATOR).collect();
  let conf = Config {
    headers: res_chunks[1].to_string(),
    addr: res_chunks[2].to_string(),
    content_length: res_chunks[3].parse()?,
    thread_count: res_chunks[4].parse()?,
    call_interval_in_ms: res_chunks[5].parse()?,
    config_fetch_interval_in_ms: res_chunks[6].parse()?,
    state: res_chunks[7].parse()?,
  };

  println!("Done fetching config {:?}.", conf);

  Ok(conf)
}
