use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::local::Local;

use std::time::Duration;

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone, Copy)]
pub struct DesiredTime(NaiveDateTime);

impl DesiredTime {
    pub fn now() -> DesiredTime {
        DesiredTime(Local::now().naive_local())
    }

    pub fn time_from_now(self) -> Duration {
        let milliseconds_from_now = Local::now().naive_local().signed_duration_since(self.0).num_milliseconds();
        if milliseconds_from_now < 0 {
            return Duration::from_millis(0);
        }
        Duration::from_millis(
            milliseconds_from_now as u64
        )
    }

    pub fn is_ready(&self) -> bool {
        let now = Local::now().naive_local();
        now > self.0
    }
}

impl Into<DesiredTime> for Duration {
    fn into(self) -> DesiredTime {
        DesiredTime(
            Local::now().naive_local().checked_add_signed(::chrono::Duration::from_std(self).unwrap()).unwrap()
        )
    }
}

impl Into<DesiredTime> for NaiveDateTime {
    fn into(self) -> DesiredTime {
        DesiredTime(
            self
        )
    }
}
