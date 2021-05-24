
pub trait RtcTrait<T>
    where
        T: DateTimeTrait
{
    type Error;

    fn set(&mut self, datetime: &T) -> Result<(), Self::Error>;
    fn get(&mut self) -> Result<T, Self::Error>;
}

/// Real-Time Clock / Calendar
pub trait DateTimeTrait {
    /// Error type
    type Error;

    /// Read the seconds.
    fn get_seconds(&self) -> Result<u8, Self::Error>;

    /// Read the minutes.
    fn get_minutes(&self) -> Result<u8, Self::Error>;

    /// Read the hours.
    fn get_hours(&self) -> Result<u8, Self::Error>;

    /// Read the day of the month [1-31].
    fn get_day(&self) -> Result<u8, Self::Error>;

    /// Read the month [1-12].
    fn get_month(&self) -> Result<u8, Self::Error>;

    /// Read the year (e.g. 2000).
    fn get_year(&self) -> Result<u16, Self::Error>;

    /// Set the seconds [0-59].
    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error>;

    /// Set the minutes [0-59].
    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error>;

    /// Set the hours.
    fn set_hours(&mut self, hours: u8) -> Result<(), Self::Error>;

    /// Set the day of month [1-31].
    fn set_day(&mut self, day: u8) -> Result<(), Self::Error>;

    /// Set the month [1-12].
    fn set_month(&mut self, month: u8) -> Result<(), Self::Error>;

    /// Set the year. (e.g. 2000)
    fn set_year(&mut self, year: u16) -> Result<(), Self::Error>;
}