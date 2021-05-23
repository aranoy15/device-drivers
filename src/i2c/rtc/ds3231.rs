
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

#[repr(u8)]
enum ControlData {
    Control = 0x0E,
    StatusReg = 0x0F
}
