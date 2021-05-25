
use embedded_hal::blocking::{i2c::{Read, Write}};
use super::traits::RtcTrait;
use super::datetime::DateTime;
use crate::i2c::rtc::traits::DateTimeTrait;
use crate::i2c::rtc::datetime::DateTimeErrors;

#[allow(dead_code)]
#[repr(u8)]
enum ControlData {
    Control = 0x0E,
    StatusReg = 0x0F
}

pub struct Rtc<I2CType> {
    i2c: I2CType,
    address: u8
}

pub enum RtcError {
    WrongValue,
    I2cError
}

impl<I2CType> Rtc<I2CType>
    where
        I2CType: Read + Write,
{
    pub fn new(i2c: I2CType, address: u8) -> Self {
        Rtc {
            i2c,
            address
        }
    }

    fn bcd_to_bin(value: u8) -> u8 {
        value - 6 * (value >> 4)
    }

    fn bin_to_bcd(value: u8) -> u8 {
        value + 6 * (value / 10)
    }

    fn write(&mut self, data: &[u8]) -> Result<(), RtcError> {
        self.i2c.write(self.address, data)
            .map_err(|_| { RtcError::I2cError })
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), RtcError> {
        self.i2c.read(self.address, data)
            .map_err(|_| { RtcError::I2cError } )
    }
}

impl From<DateTimeErrors> for RtcError {
    fn from(_: DateTimeErrors) -> Self {
        RtcError::WrongValue
    }
}

impl<I2CType> RtcTrait<DateTime> for Rtc<I2CType>
    where
        I2CType: Read + Write
{
    type Error = RtcError;

    fn set(&mut self, datetime: &DateTime) -> Result<(), Self::Error> {
        let data_to_send = [
            0_u8,
            Self::bin_to_bcd(datetime.get_seconds()?),
            Self::bin_to_bcd(datetime.get_minutes()?),
            Self::bin_to_bcd(datetime.get_hours()?),
            Self::bin_to_bcd(0_u8),
            Self::bin_to_bcd(datetime.get_day()?),
            Self::bin_to_bcd(datetime.get_month()?),
            Self::bin_to_bcd((datetime.get_year()? - 2000) as u8)
        ];

        self.write(&data_to_send)?;

        let mut status_reg = [0_u8];

        self.read(&mut status_reg)?;
        status_reg[0] &= !0x80;

        let output_data: [u8; 2] = [ControlData::StatusReg as u8, status_reg[0]];
        self.write(&output_data)?;

        Ok(())
    }

    fn get(&mut self) -> Result<DateTime, Self::Error> {
        let mut data_to_read = [0_u8; 7];

        self.write(&[0_u8])?;

        self.read(&mut data_to_read)?;

        Ok(
            DateTime::new()
                .seconds(Self::bcd_to_bin(data_to_read[0] & 0x7F))
                .minutes(Self::bcd_to_bin(data_to_read[1]))
                .hours(Self::bcd_to_bin(data_to_read[2]))
                .day(Self::bcd_to_bin(data_to_read[4]))
                .month(Self::bcd_to_bin(data_to_read[5]))
                .year(2000_u16 + (Self::bcd_to_bin(data_to_read[6]) as u16))
        )
    }
}
