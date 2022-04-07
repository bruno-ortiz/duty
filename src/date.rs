use std::fmt::{Display, Formatter};
use time::macros::time;
use time::Weekday::{Saturday, Sunday};
use time::{Date, PrimitiveDateTime};

#[derive(Debug, Copy, Clone)]
pub struct OnDutyDate {
    start_time: PrimitiveDateTime,
    end_time: PrimitiveDateTime,
}

impl OnDutyDate {
    pub fn create(base_date: Date) -> OnDutyDate {
        let start_time = PrimitiveDateTime::new(base_date, time!(19:00));
        let end_time = PrimitiveDateTime::new(base_date.next_day().unwrap(), time!(9:00));
        OnDutyDate {
            start_time,
            end_time,
        }
    }

    pub fn create_full(base_date: Date) -> OnDutyDate {
        let start_time = PrimitiveDateTime::new(base_date, time!(00:00));
        let end_time = PrimitiveDateTime::new(base_date, time!(23:59));
        OnDutyDate {
            start_time,
            end_time,
        }
    }

    pub fn create_short(base_date: Date) -> OnDutyDate {
        let start_time = PrimitiveDateTime::new(base_date, time!(19:00));
        let end_time = PrimitiveDateTime::new(base_date, time!(23:59));
        OnDutyDate {
            start_time,
            end_time,
        }
    }

    pub fn create_sadness(base_date: Date) -> OnDutyDate {
        let start_time = PrimitiveDateTime::new(base_date, time!(00:00));
        let end_time = PrimitiveDateTime::new(base_date, time!(09:00));
        OnDutyDate {
            start_time,
            end_time,
        }
    }

    pub fn is_saturday(&self) -> bool {
        self.start_time.weekday() == Saturday
    }

    pub fn is_sunday(&self) -> bool {
        self.start_time.weekday() == Sunday
    }

    pub fn start_time(&self) -> PrimitiveDateTime {
        self.start_time
    }

    pub fn end_time(&self) -> PrimitiveDateTime {
        self.end_time
    }

    pub fn duration_in_hours(&self) -> u64 {
        let seconds = (self.end_time - self.start_time).as_seconds_f64();
        (seconds / (3600.0)).ceil() as u64
    }
}

impl Display for OnDutyDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {}, end: {}", self.start_time, self.end_time)
    }
}
