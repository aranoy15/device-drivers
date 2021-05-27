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

fn crc(data: &[u8]) -> u8 {
    let mut result: u8 = 0;

    let _len = data.len();

    for &number in data {
        //result += number;
       match result.overflowing_add(number) {
           (value, _) => {
               result = value;
           }
       }
    }

    match 255_u8.overflowing_sub(result) {
        (value, _) => { result = value; }
    }

    /*
    match result.overflowing_add(1_u8) {
        (value, _) => { result = value; }
    }
    */

    result
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
        self.buffer[8] = crc(&self.buffer[0..8]);

        for &b in self.buffer.iter() {
            match nb::block!(self.serial.write(b)) {
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
                    if crc(&self.buffer[0..8]) == self.buffer[8] {
                        return Ok(());
                    }
                }
            }
        }

        Err(Mhz19Errors::ErrorRead)
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


#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::serial::{Write, Read};
    use embedded_hal::timer::CountDown;
    use std::collections::VecDeque;

    type Word = u8;

    struct DumpTimer {
        counter: u32,
        timeout: u32
    }

    impl DumpTimer {
        fn new() -> Self {
            Self {
                counter: 0_u32,
                timeout: 0_u32
            }
        }
    }

    impl CountDown for DumpTimer {
        type Time = u32;

        fn start<T>(&mut self, count: T) where
            T: Into<Self::Time> {

            self.timeout = count.into();
        }

        fn wait(&mut self) -> nb::Result<(), void::Void> {
            self.counter += 1;

            if self.counter >= self.timeout {
                return Ok(());
            }

            Err(nb::Error::WouldBlock)
        }
    }

    struct DumpSerial {
        input_data: VecDeque<u8>,
        output_data: VecDeque<u8>
    }

    impl DumpSerial {
        fn new() -> Self {
            Self {
                input_data: VecDeque::new(),
                output_data: VecDeque::new()
            }
        }

        fn process(&mut self, data: &[u8]) {
            if data[0] != 0xFF { return; }
            if data[1] != 0x01 { return; }

            match data[2] {
                0x86 => {
                    self.output_data.push_back(0xFF);
                    self.output_data.push_back(0x01);
                    self.output_data.push_back(0x04);
                    self.output_data.push_back(0xB0);
                    self.output_data.push_back(0x00);
                    self.output_data.push_back(0x00);
                    self.output_data.push_back(0x00);
                    self.output_data.push_back(0x00);

                    let (front, _) = self.output_data.as_slices();
                    let crc = crc(front);
                    self.output_data.push_back(crc);
                },
                _ => { panic!("unexpected command!"); }
            }
        }

        fn receive(&mut self, data: u8) {
            self.input_data.push_back(data);

            if self.input_data.len() == 9 {
                let mut parse_buffer: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

                for mut d in parse_buffer.iter_mut() {
                    match self.input_data.pop_front() {
                        Some(inp) => { *d = inp; },
                        _ => {}
                    }
                }

                if crc(&parse_buffer[0..8]) == parse_buffer[8] {
                    self.process(&parse_buffer);
                }
            }
        }
    }

    impl Write<Word> for DumpSerial {
        type Error = nb::Error<()>;

        fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            self.receive(word);

            Ok(())
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            Ok(())
        }
    }

    impl Read<Word> for DumpSerial {
        type Error = nb::Error<()>;

        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            if self.output_data.is_empty() {
                return Err(nb::Error::WouldBlock);
            }

            let mut result = 0_u8;
            match self.output_data.pop_front() {
                Some(d) => { result = d; },
                _ => { return Err(nb::Error::WouldBlock); }
            }

            Ok(result)
        }
    }

    type TestMhz19Type = Mhz19<DumpSerial, DumpTimer>;

    #[test]
    fn test_crc() {
        let test_data_1: [u8; 9] = [0xFF, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79];
        let test_data_2: [u8; 9] = [0xFF, 0x01, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78];
        let test_data_3: [u8; 9] = [0xFF, 0x01, 0x88, 0x07, 0xD0, 0x00, 0x00, 0x00, 0xA0];

        assert_eq!(crc(&test_data_1[0..8]), test_data_1[8]);
        assert_eq!(crc(&test_data_2[0..8]), test_data_2[8]);
        assert_eq!(crc(&test_data_3[0..8]), test_data_3[8]);
    }

    #[test]
    fn test_get_co2() {
        let serial = DumpSerial::new();
        let timer = DumpTimer::new();

        let mut mhz = TestMhz19Type::new(serial, timer);

        let co2 = mhz.get_co2().unwrap_or(0);

        assert_eq!(co2, 1200);
    }
}
