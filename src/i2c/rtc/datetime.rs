pub use rtcc::{Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike};

#[derive(Clone, Copy)]
pub struct DateTime {


    timestamp: u32,
}

const DAYS_IN_MONTH: [u8; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

enum DateTimeErrors {
    WrongValue
}

impl Rtcc for DateTime {
    type Error = DateTimeErrors;

    fn get_seconds(&mut self) -> Result<u8, Self::Error> {
        Ok(0_u8)
    }

    fn get_minutes(&mut self) -> Result<u8, Self::Error>;

    /// Read the hours.
    fn get_hours(&mut self) -> Result<Hours, Self::Error>;

    /// Read the time.
    fn get_time(&mut self) -> Result<NaiveTime, Self::Error>;

    /// Read the day of the week [1-7].
    fn get_weekday(&mut self) -> Result<u8, Self::Error>;

    /// Read the day of the month [1-31].
    fn get_day(&mut self) -> Result<u8, Self::Error>;

    /// Read the month [1-12].
    fn get_month(&mut self) -> Result<u8, Self::Error>;

    /// Read the year (e.g. 2000).
    fn get_year(&mut self) -> Result<u16, Self::Error>;

    /// Read the date.
    fn get_date(&mut self) -> Result<NaiveDate, Self::Error>;

    /// Read the date and time.
    fn get_datetime(&mut self) -> Result<NaiveDateTime, Self::Error>;

    /// Set the seconds [0-59].
    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error>;

    /// Set the minutes [0-59].
    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error>;

    /// Set the hours.
    ///
    /// Changes the operating mode to 12h/24h depending on the parameter.
    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error>;

    /// Set the time.
    fn set_time(&mut self, time: &NaiveTime) -> Result<(), Self::Error>;

    /// Set the day of week [1-7].
    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error>;

    /// Set the day of month [1-31].
    fn set_day(&mut self, day: u8) -> Result<(), Self::Error>;

    /// Set the month [1-12].
    fn set_month(&mut self, month: u8) -> Result<(), Self::Error>;

    /// Set the year. (e.g. 2000)
    fn set_year(&mut self, year: u16) -> Result<(), Self::Error>;

    /// Set the date.
    fn set_date(&mut self, date: &NaiveDate) -> Result<(), Self::Error>;

    /// Set the date and time.
    ///
    /// This will set the hour operating mode to 24h and the weekday to the
    /// day number starting from Sunday = 1.
    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error>;
}
