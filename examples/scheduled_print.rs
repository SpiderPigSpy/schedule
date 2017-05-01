extern crate schedule;

use schedule::*;

use std::time::Duration;

fn main() {
    let schedule = Schedule::<String>::with_consumer(|data| println!("{}", data));
    schedule.send("Sent first, but consumed second", Duration::from_millis(100)).unwrap();
    schedule.send_now("Sent second, but consumed first").unwrap();
    ::std::thread::sleep(Duration::from_millis(200));
}
