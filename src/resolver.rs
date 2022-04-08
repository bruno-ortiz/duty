use crate::date::OnDutyDate;
use crate::duty::{is_on_duty_in_previous_day, DutyAttendant, OnDutyEntry};
use std::cell::RefCell;
use std::rc::Rc;

pub trait OnDutyDateResolver {
    fn can_resolve(&self, date: OnDutyDate) -> bool;
    fn resolve(&self, date: OnDutyDate, attendants: &[Rc<RefCell<DutyAttendant>>]) -> OnDutyEntry;
}

pub struct CompositeOnDutyResolver {
    resolvers: Vec<Box<dyn OnDutyDateResolver + Send + Sync>>,
}

impl CompositeOnDutyResolver {
    pub fn new() -> Self {
        Self {
            resolvers: vec![
                Box::new(WeekdayResolver),
                Box::new(SaturdayResolver),
                Box::new(SundayResolver),
            ],
        }
    }
}

impl OnDutyDateResolver for CompositeOnDutyResolver {
    fn can_resolve(&self, date: OnDutyDate) -> bool {
        self.resolvers.iter().any(|r| r.can_resolve(date))
    }

    fn resolve(&self, date: OnDutyDate, attendants: &[Rc<RefCell<DutyAttendant>>]) -> OnDutyEntry {
        match self.resolvers.iter().find(|r| r.can_resolve(date)) {
            Some(r) => r.resolve(date, attendants),
            None => (None, date),
        }
    }
}

struct SaturdayResolver;

impl OnDutyDateResolver for SaturdayResolver {
    fn can_resolve(&self, date: OnDutyDate) -> bool {
        date.is_saturday()
    }

    fn resolve(&self, date: OnDutyDate, attendants: &[Rc<RefCell<DutyAttendant>>]) -> OnDutyEntry {
        if let Some(a) = attendants
            .iter()
            .filter(|d| !is_on_duty_in_previous_day(date, &d.borrow()))
            .filter(|d| !d.borrow().has_saturday())
            .min()
        {
            let mut attendant = a.borrow_mut();
            attendant.update(&date);
            (Some(a.clone()), date)
        } else {
            (None, date)
        }
    }
}

struct SundayResolver;

impl OnDutyDateResolver for SundayResolver {
    fn can_resolve(&self, date: OnDutyDate) -> bool {
        date.is_sunday()
    }

    fn resolve(&self, date: OnDutyDate, attendants: &[Rc<RefCell<DutyAttendant>>]) -> OnDutyEntry {
        if let Some(a) = attendants
            .iter()
            .filter(|d| !is_on_duty_in_previous_day(date, &d.borrow()))
            .filter(|d| !d.borrow().has_sunday())
            .min()
        {
            let mut attendant = a.borrow_mut();
            attendant.update(&date);
            (Some(a.clone()), date)
        } else {
            (None, date)
        }
    }
}

struct WeekdayResolver;

impl OnDutyDateResolver for WeekdayResolver {
    fn can_resolve(&self, date: OnDutyDate) -> bool {
        !date.is_sunday() && !date.is_saturday()
    }

    fn resolve(&self, date: OnDutyDate, attendants: &[Rc<RefCell<DutyAttendant>>]) -> OnDutyEntry {
        if let Some(a) = attendants
            .iter()
            .filter(|d| !is_on_duty_in_previous_day(date, &d.borrow()))
            .min()
        {
            let mut attendant = a.borrow_mut();
            attendant.update(&date);
            (Some(a.clone()), date)
        } else {
            (None, date)
        }
    }
}
