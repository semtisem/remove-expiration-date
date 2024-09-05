use std::time::SystemTime;

use chrono::{DateTime, Utc};

#[allow(unused)]
pub fn to_datetime_utc(time: SystemTime) -> DateTime<Utc> {
    DateTime::from(time)
}
