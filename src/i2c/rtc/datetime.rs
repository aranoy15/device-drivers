use super::traits::DateTimeTrait;

#[derive(Clone, Copy)]
pub struct DateTime {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day: u8,
    month: u8,
    year: u16
}

impl DateTime {
    pub fn new() -> Self {
        DateTime {
            seconds: 0_u8,
            minutes: 0_u8,
            hours: 0_u8,
            day: 1_u8,
            month: 1_u8,
            year: 1970_u16
        }
    }

    pub fn seconds(mut self, seconds: u8) -> Self {
        self.seconds = seconds;
        self
    }

    pub fn minutes(mut self, minutes: u8) -> Self {
        self.minutes = minutes;
        self
    }

    pub fn hours(mut self, hours: u8) -> Self {
        self.hours = hours;
        self
    }

    pub fn day(mut self, day: u8) -> Self {
        self.day = day;
        self
    }

    pub fn month(mut self, month: u8) -> Self {
        self.month = month;
        self
    }

    pub fn year(mut self, year: u16) -> Self {
        self.year = year;
        self
    }
}

#[allow(dead_code)]
const DAYS_IN_MONTH: [u8; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

pub enum DateTimeErrors {
    WrongValue
}

impl DateTimeTrait for DateTime {
    type Error = DateTimeErrors;

    /// Read the seconds.
    fn get_seconds(&self) -> Result<u8, Self::Error> {
        Ok(self.minutes)
    }

    /// Read the minutes.
    fn get_minutes(&self) -> Result<u8, Self::Error> {
        Ok(self.minutes)
    }

    /// Read the hours.
    fn get_hours(&self) -> Result<u8, Self::Error> {
        Ok(self.hours)
    }

    /// Read the day of the month [1-31].
    fn get_day(&self) -> Result<u8, Self::Error> {
        Ok(self.day)
    }

    /// Read the month [1-12].
    fn get_month(&self) -> Result<u8, Self::Error> {
        Ok(self.month)
    }

    /// Read the year (e.g. 2000).
    fn get_year(&self) -> Result<u16, Self::Error> {
        Ok(self.year)
    }

    /// Set the seconds [0-59].
    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error> {
        if seconds > 59 { return Err(Self::Error::WrongValue); }

        self.seconds = seconds;
        Ok(())
    }

    /// Set the minutes [0-59].
    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error> {
        if minutes > 59 { return Err(Self::Error::WrongValue); }

        self.minutes = minutes;
        Ok(())
    }

    /// Set the hours.
    fn set_hours(&mut self, hours: u8) -> Result<(), Self::Error> {
        if hours > 23 { return Err(Self::Error::WrongValue); }

        self.hours = hours;
        Ok(())
    }

    /// Set the day of month [1-31].
    fn set_day(&mut self, day: u8) -> Result<(), Self::Error> {
        if day > 31 || day == 0 { return Err(Self::Error::WrongValue); }

        self.day = day;
        Ok(())
    }

    /// Set the month [1-12].
    fn set_month(&mut self, month: u8) -> Result<(), Self::Error> {
        if month > 12 || month == 0 { return Err(Self::Error::WrongValue); }

        self.month = month;
        Ok(())
    }

    /// Set the year. (e.g. 2000)
    fn set_year(&mut self, year: u16) -> Result<(), Self::Error> {
        if year < 1970 { return Err(Self::Error::WrongValue); }

        self.year = year;
        Ok(())
    }
}
