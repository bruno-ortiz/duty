use std::str::FromStr;
use time::{format_description, Date, Month};

#[derive(Copy, Clone)]
pub struct Period(u8, Month, i32);

impl Period {
    pub fn new(day: u8, month: Month, year: i32) -> Self {
        Self(day, month, year)
    }

    pub fn day(&self) -> u8 {
        self.0
    }

    pub fn month(&self) -> Month {
        self.1
    }

    pub fn year(&self) -> i32 {
        self.2
    }
}

impl FromStr for Period {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = format_description::parse("[year]-[month]-[day]")?;
        Ok(Date::parse(s, &format).map(|d| Period::new(d.day(), d.month(), d.year()))?)
    }
}
