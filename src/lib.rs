extern crate chrono;
#[macro_use] extern crate log;

mod time;
mod queue;

pub use time::*;

use queue::*;

use std::sync::mpsc::{Sender, Receiver, SendError, RecvTimeoutError};
use std::sync::mpsc::channel;
use std::thread;

pub trait Consumer<T> {
     fn consume(&self, t: T);
}


pub struct Schedule<T: Send> {
    send: Sender<(T, DesiredTime)>,
}

struct ScheduledExecutor<T, S: Consumer<T>> {
    rcv: Receiver<(T, DesiredTime)>,
    queue: Queue<T>,
    consumer: S
}

impl<T: Send> Schedule<T> {
    pub fn with_consumer<S: Consumer<T>>(consumer: S) -> Schedule<T> {
        let (send, rcv) = channel();
        
        ScheduledExecutor {
            rcv: rcv,
            queue: Queue::new(),
            consumer: consumer
        }.run();

        Schedule {
            send: send
        }
    }

    pub fn send_now(&self, message: T) -> Result<(), SendError<T>> {
        self.send
            .send((message, DesiredTime::now()))
            .map_err(|err| SendError((err.0).0))
    }

    pub fn send<S: Into<DesiredTime>>(&self, message: T, time: S) -> Result<(), SendError<T>> {
        self.send
            .send((message, time.into()))
            .map_err(|err| SendError((err.0).0))
    }
}

impl<T, S: Consumer<T>> ScheduledExecutor<T, S> {
    fn run(mut self) {
        thread::spawn(move || {
            loop {
                match send_while_next_is_available(&mut self.queue, &self.consumer) {
                    Some(desired_time) => {
                        match self.rcv.recv_timeout(desired_time.time_from_now()) {
                            Ok((data, desired_time)) => self.queue.put(data, desired_time),
                            Err(RecvTimeoutError::Timeout) => continue,
                            Err(RecvTimeoutError::Disconnected) => break
                        }
                    },
                    None => {
                        match self.rcv.recv() {
                            Ok((data, desired_time)) => self.queue.put(data, desired_time),
                            Err(_) => break
                        }
                    }
                }
            }
            info!("ScheduledExecutor ended");
        });
    } 
    
}

impl<S, T> Consumer<T> for S where S: Fn(T) {
    fn consume(&self, data: T) {
        self(data)
    }
}

fn send_while_next_is_available<T, S: Consumer<T>>(queue: &mut Queue<T>, consumer: &S) -> Option<DesiredTime> {
    loop {
        match queue.next() {
            NextItem::After(desired_time) => return Some(desired_time),
            NextItem::Never => return None,
            NextItem::Now(data) => consumer.consume(data)
        }
    }
}

#[cfg(test)]
mod tests {
    use ::chrono::Duration;
    use ::chrono::offset::local::Local;

    use super::*;

    use std::sync::mpsc::channel;

    #[test]
    fn consumes_data_in_correct_order() {
        //given
        let (send, recv) = channel();
        let schedule = Schedule::with_consumer(move |data| send.send(data).unwrap());
        let now = Local::now().naive_local();
        //when
        schedule.send(1, now.checked_add_signed(Duration::milliseconds(10)).unwrap()).unwrap();
        schedule.send(0, now).unwrap();
        //then
        assert_eq!(recv.recv().unwrap(), 0);
        assert_eq!(recv.recv().unwrap(), 1);
    }
}
