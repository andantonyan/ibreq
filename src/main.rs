use ibreq::*;
use std::{thread, time::Duration, time::Instant};

static CONF_ADDR: &str = "localhost:3000";

fn main() {
  loop {
    match get_conf(CONF_ADDR) {
      Ok(conf) => {
        let start = Instant::now();
        let config_fetch_interval = Duration::from_millis(conf.config_fetch_interval_in_ms);
        let call_interval = Duration::from_millis(conf.call_interval_in_ms);

        if conf.enabled {
          'inner: loop {
            let mut threads = vec![];

            for i in 0..conf.thread_count {
              let conf = conf.clone();

              let thread = thread::spawn(move || {
                return match call(&conf) {
                  Ok(res) => {
                    debug!(format!("Thread:\"{}\", Response\n{:?}", i, res.headers));
                    thread::sleep(call_interval);
                  }
                  Err(err) => {
                    debug!(format!("Thread:\"{}\", Unable to call - {:?}.", i, err));
                    thread::sleep(call_interval);
                  }
                };
              });

              thread::sleep(Duration::from_millis(10));

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
      }
      Err(err) => {
        thread::sleep(Duration::from_millis(1000));
        debug!(format!("Unable to get config - {:?}.", err));
      }
    }
  }
}
