use crate::duty::build_on_duty_days;
use crate::writer::{CsvWriter, Writer};
use std::fs::File;
use std::io::stdout;
use time::Month::April;

mod date;
mod date_ext;
mod duty;
mod writer;

fn main() {
    let entries = build_on_duty_days(
        &[
            "FERNANDO FERNANDES",
            "PEDRO THIAGO MACIEL",
            "VALDINEI REIS DA SILVA",
            "RODRIGO MAC KNIGHT POLONI",
        ],
        (7, April, 2022),
    );
    CsvWriter
        .write(
            // File::create("/Users/bruno/Documents/test.csv").unwrap(),
            stdout(),
            entries,
        )
        .expect("No errors");
}
