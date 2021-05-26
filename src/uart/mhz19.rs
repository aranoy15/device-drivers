use embedded_hal::serial::{Write, Read};
use embedded_hal::timer::CountDown;

#[allow(dead_code)]
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

pub enum Mhz19Errors {
    ErrorWrite,
    ErrorRead,
    WrongValue
}

pub trait Mhz19Trait {
    type Error;

    fn init(&mut self) -> Result<(), Self::Error>;
    fn get_co2(&mut self) -> Result<u16, Self::Error>;
    fn set_auto_calibration(&mut self, state: bool) -> Result<(), Self::Error>;
    fn set_range(&mut self) -> Result<(), Self::Error>;
}

pub struct Mhz19<SerialType, TimerType>
    where
        SerialType: Read<u8> + Write<u8>,
        TimerType: CountDown<Time = u32>
{
    serial: SerialType,
    timer: TimerType,
    buffer: [u8; 9]
}

impl<SerialType, TimerType> Mhz19<SerialType, TimerType>
    where
        SerialType: Read<u8> + Write<u8>,
        TimerType: CountDown<Time = u32>
{
    pub fn new(serial: SerialType, timer: TimerType) -> Self {
        Mhz19 {
            serial,
            timer,
            buffer: [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8]
        }
    }

    fn command(&mut self, cmd: u8, data: [u8; 5]) -> Result<(), Mhz19Errors> {
        self.buffer[0] = 0xFF;
        self.buffer[1] = 0x01;
        self.buffer[2] = cmd;
        self.buffer[3] = data[0];
        self.buffer[4] = data[1];
        self.buffer[5] = data[2];
        self.buffer[6] = data[3];
        self.buffer[7] = data[4];
        self.buffer[8] = self.crc();

        for b in self.buffer.iter() {
            match nb::block!(self.serial.write(*b)) {
                Err(_) => { return Err(Mhz19Errors::ErrorWrite); }
                _ => {}
            }
        }

        Ok(())
    }

    fn response(&mut self) -> Result<(), Mhz19Errors> {
        self.timer.start(1000_u32);

        let mut count: usize = 0;
        while let Err(_) = self.timer.wait() {
            if let Ok(data) = self.serial.read() {
                self.buffer[count] = data;
                count += 1;

                if count == 9 {
                    if self.crc() == self.buffer[8] {
                        return Ok(());
                    }
                }
            }
        }

        Err(Mhz19Errors::ErrorRead)
    }

    fn crc(&self) -> u8 {
        let mut result = 0_u8;

        for number in &self.buffer[0..8] {
            result += *number;
        }

        result = 0xFF - result;
        result += 1;

        result
    }
}

impl<SerialType, TimerType> Mhz19Trait for Mhz19<SerialType, TimerType>
    where
        SerialType: Read<u8> + Write<u8>,
        TimerType: CountDown<Time = u32>
{
    type Error = Mhz19Errors;

    fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn get_co2(&mut self) -> Result<u16, Self::Error> {
        let data: [u8; 5] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8];

        if let Err(error) = self.command(0x86, data) {
            return Err(error);
        }

        if let Err(error)  = self.response() {
            return Err(error);
        }

        Ok(((self.buffer[2] as u16) << 8_u16) | (self.buffer[3] as u16))
    }

    fn set_auto_calibration(&mut self, state: bool) -> Result<(), Self::Error> {
        let mut state_val: u8 = 0x00;
        if state { state_val = 0xA0; }

        let data= [state_val, 0_u8, 0_u8, 0_u8, 0_u8];

        self.command(Commands::OnOffAutoCalibration as u8, data)
    }

    fn set_range(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
