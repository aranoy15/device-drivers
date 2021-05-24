use embedded_hal::serial::{Write, Read};

#[repr(u8)]
enum Commands {
    ReadConcentration = 0x86,
    CalibrateZeroPoint = 0x87,
    CalibrateSpanPoint = 0x88,
    OnOffAutoCalibration = 0x79,
    DetectionRangeSetting = 0x99
}

pub enum Range {
    _1000,
    _2000,
    _3000,
    _5000,
    _10000
}

pub enum Errors {
    NoAnswer,
    WrongValue
}

pub trait Trait {
    fn init(&mut self) -> Result<(), Errors>;
    fn get_co2(&mut self) -> Result<u16, Errors>;
    fn set_auto_calibration(&mut self) -> Result<(), Errors>;
    fn set_range(&mut self) -> Result<(), Errors>;
}

pub struct Mhz19<SerialType>
    where
        SerialType: Read<u8> + Write<u8>
{

}
