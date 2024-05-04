#![allow(unused)]

use chrono::*;

fn main() -> Result<()> {
    let temp_date = "May 21, 2021";
    let now = now_utc();
    let formatted = format_time(now);
    println!(
        "Before - {:#?}\nAfter - {:#?}",
        formatted,
        parse_utc(&formatted)
    );

    let parsed_date = NaiveDate::parse_from_str(temp_date, "%b %d, %Y");
    println!("{:?}", parsed_date);
    println!("Parsed - {:?}", format_time(parsed_date));
    Ok(())
}
use time::{Duration, OffsetDateTime};

use time::format_description::well_known::Rfc3339;

fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // TODO: need to check if safe.
}

fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

// region: ---- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToDateParse(String),
    CannotParseFromString,
}

// region: ---- Error boilerplate.

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// endregion: ---- Error boilerplate.

// endregion: ---- Error
