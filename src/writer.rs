use crate::duty::OnDutyEntry;
use std::io::Write;

pub trait Writer {
    fn write<W: Write>(&self, writer: W, entries: Vec<OnDutyEntry>) -> anyhow::Result<()>;
}

pub struct CsvWriter;

impl Writer for CsvWriter {
    fn write<W: Write>(&self, mut writer: W, entries: Vec<OnDutyEntry>) -> anyhow::Result<()> {
        for entry in entries {
            let date = entry.1;
            let start_time = date.start_time();
            let end_time = date.end_time();
            if let Some(duty_attendant) = entry.0 {
                let duty_attendant = duty_attendant.as_ref().borrow();
                write!(
                    writer,
                    "{},{},{} {},{} {},{},{}\r\n",
                    start_time.date(),
                    duty_attendant.name,
                    start_time.weekday(),
                    start_time.time(),
                    end_time.weekday(),
                    end_time.time(),
                    date.duration_in_hours(),
                    date.is_saturday() || date.is_sunday()
                )?
            } else {
                write!(
                    writer,
                    "{},,{} {},{} {},{},{}\r\n",
                    start_time.date(),
                    start_time.weekday(),
                    start_time.time(),
                    end_time.weekday(),
                    end_time.time(),
                    date.duration_in_hours(),
                    date.is_saturday() || date.is_sunday()
                )?
            }
        }
        Ok(())
    }
}
