use time::{Date, Weekday};

pub trait DateExt {
    fn is_rest_day(&self) -> bool;
    fn has_upcoming_rest_day(&self) -> bool;
    fn has_upcoming_working_day(&self) -> bool;
}

impl DateExt for Date {
    fn is_rest_day(&self) -> bool {
        self.weekday() == Weekday::Saturday || self.weekday() == Weekday::Sunday
    }

    fn has_upcoming_rest_day(&self) -> bool {
        let next_day = self.next_day().unwrap();
        next_day.is_rest_day()
    }

    fn has_upcoming_working_day(&self) -> bool {
        !self.has_upcoming_rest_day()
    }
}