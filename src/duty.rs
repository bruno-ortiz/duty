use crate::date::OnDutyDate;
use crate::date_ext::DateExt;
use anyhow::anyhow;
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;
use time::{Date, Month};

pub type OnDutyEntry = (Option<Rc<RefCell<DutyAttendant>>>, OnDutyDate);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DutyAttendant {
    pub name: String,
    on_duty_dates: Vec<Date>,
    total_hours: u64,
    has_saturday: bool,
    has_sunday: bool,
}

impl DutyAttendant {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            on_duty_dates: Vec::new(),
            total_hours: 0,
            has_saturday: false,
            has_sunday: false,
        }
    }

    pub fn is_on_duty(&self, date: Date) -> bool {
        self.on_duty_dates.contains(&date)
    }

    pub fn update(&mut self, date: &OnDutyDate) {
        self.on_duty_dates.push(date.start_time().date());
        self.total_hours += date.duration_in_hours();
        if !self.has_saturday {
            self.has_saturday = date.is_saturday();
        }
        if !self.has_sunday {
            self.has_sunday = date.is_sunday();
        }
    }
    pub fn total_hours(&self) -> u64 {
        self.total_hours
    }
}

impl Ord for DutyAttendant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_hours.cmp(&other.total_hours)
    }
}

impl PartialOrd for DutyAttendant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_hours.partial_cmp(&other.total_hours)
    }
}

pub fn build_on_duty_days(participants: &[&str], period: (u8, Month, i32)) -> Vec<OnDutyEntry> {
    let mut attendants: Vec<_> = participants
        .iter()
        .map(|&name| Rc::new(RefCell::new(DutyAttendant::new(name))))
        .collect();

    get_on_duty_dates(period.0, period.1, period.2)
        .unwrap()
        .into_iter()
        .map(|duty_date| {
            let entry = if duty_date.is_saturday() {
                if let Some(a) = attendants.iter_mut().find(|d| {
                    let attendant = d.borrow();
                    !attendant.has_saturday && !is_on_duty_in_previous_day(duty_date, &attendant)
                }) {
                    let mut attendant = a.as_ref().borrow_mut();
                    attendant.update(&duty_date);
                    (Some(a.clone()), duty_date)
                } else {
                    (None, duty_date)
                }
            } else if duty_date.is_sunday() {
                if let Some(a) = attendants.iter_mut().find(|a| {
                    let attendant = a.borrow();
                    !a.borrow().has_sunday && !is_on_duty_in_previous_day(duty_date, &attendant)
                }) {
                    a.as_ref().borrow_mut().update(&duty_date);
                    (Some(a.clone()), duty_date)
                } else {
                    (None, duty_date)
                }
            } else if let Some(a) = attendants
                .iter_mut()
                .filter(|a| {
                    let attendant = a.borrow();
                    !is_on_duty_in_previous_day(duty_date, &attendant)
                })
                .min_by(|d1, d2| d1.borrow().total_hours().cmp(&d2.borrow().total_hours()))
            {
                a.as_ref().borrow_mut().update(&duty_date);
                (Some(a.clone()), duty_date)
            } else {
                (None, duty_date)
            };
            attendants.sort();
            entry
        })
        .collect()
}

fn get_on_duty_dates(day: u8, month: Month, year: i32) -> anyhow::Result<Vec<OnDutyDate>> {
    let mut base_date = Date::from_calendar_date(year, month, day)?;

    let mut dates = Vec::with_capacity(30);
    while base_date.month() == month {
        if base_date.is_rest_day() {
            dates.push(OnDutyDate::create_full(base_date));
            if base_date.has_upcoming_working_day() {
                dates.push(OnDutyDate::create_sadness(base_date.next_day().unwrap()))
            }
        } else if base_date.has_upcoming_rest_day() {
            dates.push(OnDutyDate::create_short(base_date))
        } else {
            dates.push(OnDutyDate::create(base_date))
        }
        base_date = base_date
            .next_day()
            .ok_or_else(|| anyhow!("Unexpected error"))?;
    }
    Ok(dates)
}

fn is_on_duty_in_previous_day(duty_date: OnDutyDate, attendant: &Ref<DutyAttendant>) -> bool {
    let previous_day = duty_date.start_time().date().previous_day().unwrap();
    attendant.is_on_duty(previous_day)
}

#[cfg(test)]
mod tests {
    use crate::duty::get_on_duty_dates;
    use time::Month::April;

    #[test]
    fn it_can_generate_on_duty_days() {
        let dates = get_on_duty_dates(2, April, 2022).expect("Expected on duty dates");

        for date in dates {
            println!("{date}");
        }
    }
}
