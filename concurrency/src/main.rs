use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

const SLEEP: Duration = Duration::from_secs(2);
const PING: String = String::from("ping");
const PONG: String = String::from("pong");
const COUNTER_RESET: u32 = 100;

struct Runner {
    counter: Arc::new(Mutex::new(100)),
}

impl Runner {
    fn run(self: &Self,
           sender: &SyncSender<String>,
           receiver: &Receiver<String>,
    ) {
        thread::spawn(|| {
            if self.should_init() {
                self.handle(sender, PING);
            }
            for received in receiver {
                thread::sleep(SLEEP);
                match received {
                    PING => { self.handle(sender, PONG) }
                    PONG => { self.handle(sender, PING) }
                }
            };
        });
    }

    fn handle(self: &Self,
              sender: &SyncSender<String>,
              res: String,
    ) {
        let mut counter = self.counter.lock().unwrap();
        if counter.eq(&0) {
            return;
        }
        sender.send(res);
        *counter -= 1;
    }

    fn should_init(self: &Self) -> bool {
        self.counter.lock()
            .unwrap()
            .eq(&COUNTER_RESET)
    }
}

fn main() {
    let (s1, r1) = std::sync::mpsc::sync_channel(1);
    let (s2, r2) = std::sync::mpsc::sync_channel(1);
    let runner = Runner { counter: Mutex::new(100) };
    runner.run(&s1, &r2);
    runner.run(&s2, &r1);
}
