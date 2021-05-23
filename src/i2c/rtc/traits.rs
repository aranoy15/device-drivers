
pub trait RtcTrait {
}

/// Real-Time Clock / Calendar
pub trait DateTimeTrait {
    /// Error type
    type Error;

    /// Read the seconds.
    fn get_seconds(&mut self) -> Result<u8, Self::Error>;

    /// Read the minutes.
    fn get_minutes(&mut self) -> Result<u8, Self::Error>;

    /// Read the hours.
    fn get_hours(&mut self) -> Result<u8, Self::Error>;

    /// Read the day of the month [1-31].
    fn get_day(&mut self) -> Result<u8, Self::Error>;

    /// Read the month [1-12].
    fn get_month(&mut self) -> Result<u8, Self::Error>;

    /// Read the year (e.g. 2000).
    fn get_year(&mut self) -> Result<u16, Self::Error>;

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