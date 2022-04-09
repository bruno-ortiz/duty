use crate::duty::OnDutyDaysFactory;
use crate::period::Period;
use crate::writer::{CsvWriter, Writer};
use clap::Parser;
use std::env;
use std::fs::File;
use std::io::stdout;
use std::io::Write;
use std::path::PathBuf;

mod date;
mod date_ext;
mod duty;
mod period;
mod resolver;
mod writer;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap[short, long]]
    participants: Vec<String>,
    #[clap[long]]
    stdout: bool,
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    file: Option<PathBuf>,
    #[clap[short, long]]
    starting_period: Period,
}

fn main() {
    let cli = Cli::parse();
    let participants: Vec<_> = cli.participants.iter().map(AsRef::as_ref).collect();
    let factory = OnDutyDaysFactory::new();
    let entries = factory.build_on_duty_days(&participants, cli.starting_period);
    let mut buff: Vec<u8> = Vec::new();
    CsvWriter.write(&mut buff, entries).expect("No errors");
    if cli.stdout || cli.file.is_none() {
        stdout().write_all(&buff).unwrap();
    }
    if let Some(path) = cli.file {
        let path = if path.is_absolute() {
            path
        } else {
            env::current_dir().unwrap().join(path)
        }
        .with_extension("csv");
        println!("writing to: {:?}", path);
        File::create(path).unwrap().write_all(&buff).unwrap();
    }
}
