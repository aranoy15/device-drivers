use embedded_hal::blocking::{i2c::{Write}, delay::DelayMs};

const BACKLIGHT: u8 = 0b0000_1000;
const NO_BACKLIGHT: u8 = 0b0000_0000;

const ENABLE: u8 = 0b0000_0100;
const REGISTER_SELECT: u8 = 0b0000_0001;
const DISPLAY_CONTROL: u8 = 0b0000_1000;

const ONE_LINE: u8 = 0b0000_0000;
const TWO_LINE: u8 = 0b0000_1000;
const FOUR_BIT_MODE: u8 = 0b0000_0000;
const FIVE_X8_DOTS: u8 = 0b0000_0000;
const FIVE_X10_DOTS: u8 = 0b0000_0100;

const FUNCTION_SET: u8 = 0b0010_0000;

const DISPLAY_ON: u8 = 0b0000_0100;
const CURSOR_OFF: u8 = 0b0000_0000;
const BLINK_OFF: u8 = 0b0000_0000;

const ENTRY_LEFT: u8 = 0b0000_0010;
const ENTRY_SHIFT_DECREMENT: u8 = 0b0000_0000;
const ENTRY_MODE_SET: u8 = 0b0000_0100;

const RETURN_HOME: u8 = 0b0000_0010;

const SET_DRAM_ADDR: u8 = 0b1000_0000;
const SET_CRAM_ADDR: u8 = 0b0100_0000;

const INITIALIZE_4BIT: u8 = 0x33;

pub trait LcdTrait {
    fn init(&mut self) -> Result<(), ()>;
    fn clear(&mut self) -> Result<(), ()>;
    fn reset(&mut self) -> Result<() ,()>;
    fn backlight(&mut self) -> Result<(), ()>;
    fn no_backlight(&mut self) -> Result<(), ()>;
    fn display(&mut self) -> Result<(), ()>;
    fn no_display(&mut self) -> Result<(), ()>;
    fn home(&mut self) -> Result<(), ()>;
    fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), ()>;
    fn write_char(&mut self, data: char) -> Result<(), ()>;
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), ()>;
    fn write_str(&mut self, data: &str) -> Result<(), ()>;
    fn create_char(&mut self, location: u8, char_map: &[u8; 8]) -> Result<(), ()>;
    fn write_custom_char(&mut self, location: u8) -> Result<(), ()>;
}

/// Lcd with i2c converter
///
/// # Example
///
/// ```
/// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
/// let mut lcd = Lcd::new(
///         i2c,
///         0x27,
///         led_delay
///     )
///     .columns(20)
///     .rows(4)
///     .char_size(1)
///     .build();
/// lcd.init().unwrap();
/// lcd.reset().unwrap();
/// lcd.clear().unwrap();
/// ```
///
#[derive(Default)]
pub struct Lcd<I2CType, DelayType> {
    i2c: I2CType,
    address: u8,
    delay: DelayType,

    display_function: u8,
    display_control: u8,
    display_mode: u8,
    cols: u8,
    rows: u8,
    char_size: u8,
    back_light_val: u8,
}

impl<I2cType, DelayType> Lcd<I2cType, DelayType>
    where
        I2cType: Write,
        DelayType: DelayMs<u16>
{
    /// Return new lcd instance
    ///
    /// # Arguments
    /// * `i2c` - i2c for sending data
    /// * `address` - address of device (example 0x27)
    /// * `delay` - variable for call delay_ms
    ///
    /// # Example
    /// ```
    /// use device_drivers::i2c::lcd::{Lcd};
    /// let lcd = Lcd::new(i2c, 0x27, delay).build();
    /// ```
    pub fn new(
        i2c: I2cType,
        address: u8,
        delay: DelayType,
    ) -> Self {
        Lcd {
            i2c,
            address,
            delay,
            display_function: 0u8,
            display_control: 0u8,
            display_mode: 0u8,
            cols: 16u8,
            rows: 2u8,
            char_size: 1u8,
            back_light_val: BACKLIGHT,
        }
    }

    /// Set count of columns in lcd
    ///
    /// # Arguments
    ///
    /// * `cols` - count of columns
    ///
    /// # Return
    ///
    /// * `Self` - lcd instance
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{Lcd};
    /// let lcd = Lcd::new(i2c, 0x27, delay)
    ///     .columns(20)
    ///     .build();
    /// ```
    pub fn columns(mut self, cols: u8) -> Self {
        self.cols = cols;
        self
    }

    /// Set count of rows in lcd
    ///
    /// # Arguments
    ///
    /// * `cols` - count of rows
    ///
    /// # Return
    /// * `Self` - lcd instance
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{Lcd};
    /// let lcd = Lcd::new(i2c, 0x27, delay)
    ///     .rows(4)
    ///     .build();
    /// ```
    pub fn rows(mut self, rows: u8) -> Self {
        self.rows = rows;
        self
    }

    /// Set char size for lcd
    ///
    /// # Arguments
    ///
    /// * `char_size` - char size for lcd
    ///
    /// # Return
    ///
    /// * `Self` - lcd instance
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{Lcd};
    /// let lcd = Lcd::new(i2c, 0x27, delay)
    ///     .char_size(1)
    ///     .build();
    /// ```
    pub fn char_size(mut self, char_size: u8) -> Self {
        self.char_size = char_size;
        self
    }

    /// Complete configure lcd
    ///
    /// # Return
    ///
    /// * `Self` - lcd instance
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{Lcd};
    /// let lcd = Lcd::new(i2c, 0x27, delay)
    ///     .build();
    /// ```
    pub fn build(self) -> Self {
        self
    }

    /// Send 4 bit to i2c expander
    ///
    /// # Arguments
    ///
    /// * `nibble` - 4 bit to write in expander
    /// * `data` - is data or command
    ///
    fn expander_write(&mut self, nibble: u8, data: bool) {
        let rs = match data {
            false => 0u8,
            true => REGISTER_SELECT
        };

        let byte = nibble | rs | self.back_light_val;

        let _ = self.i2c.write(self.address, &[byte, byte | ENABLE]);
        self.delay.delay_ms(2u16);
        let _ = self.i2c.write(self.address, &[byte]);
    }

    /// Send byte to i2c expander
    ///
    /// # Arguments
    ///
    /// * `byte` - 8 bit to write
    /// * `data` - is data or command
    fn write(&mut self, byte: u8, data: bool) -> Result<(), ()> {
        let upper_nibble = byte & 0xF0;
        self.expander_write(upper_nibble, data);

        let lower_nibble = (byte & 0x0F) << 4;
        self.expander_write(lower_nibble, data);

        Ok(())
    }

    /// Send byte like data
    ///
    /// # Arguments
    ///
    /// * `byte` - 8 bit to write
    fn write_byte(&mut self, byte: u8) -> Result<(), ()> {
        self.write(byte, true)?;

        self.delay.delay_ms(1_u16);

        Ok(())
    }

    /// Send byte like command
    ///
    /// # Arguments
    ///
    /// * `cmd` - 8 bit to write
    fn command(&mut self, cmd: u8) -> Result<(), ()> {
        self.write(cmd, false)?;

        self.delay.delay_ms(1_u16);

        Ok(())
    }
}

impl<I2cType, DelayType> LcdTrait for Lcd<I2cType, DelayType>
    where
        I2cType: Write,
        DelayType: DelayMs<u16>
{
    /// Init lcd display
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
    ///
    /// let mut lcd = Lcd::new(i2c, 0x27, delay).build();
    /// lcd.init().unwrap();
    /// ```
    fn init(&mut self) -> Result<(), ()> {
        self.display_function = FOUR_BIT_MODE | ONE_LINE | FIVE_X8_DOTS;

        if self.rows > 1 {
            self.display_function |= TWO_LINE;
        }

        if self.char_size != 0 && self.rows == 1 {
            self.display_function |= FIVE_X10_DOTS;
        }

        self.delay.delay_ms(15_u16);

        self.write(INITIALIZE_4BIT, false)?;
        self.delay.delay_ms(5u16);

        self.command(0x32)?;

        self.command(0x28)?;

        // Clear display

        self.command(0x0E)?;

        // Move the cursor to beginning of first line

        self.command(0x01)?;

        self.command(0x80)?;

        self.command(FUNCTION_SET | self.display_function)?;

        self.display_control = DISPLAY_ON | CURSOR_OFF | BLINK_OFF;
        self.display()?;

        self.clear()?;

        self.display_mode = ENTRY_LEFT | ENTRY_SHIFT_DECREMENT;
        self.command(ENTRY_MODE_SET | self.display_mode)?;

        self.home()?;

        Ok(())
    }

    /// Clear lcd display
    fn clear(&mut self) -> Result<(), ()> {
        self.command(0b0000_0001)?;

        Ok(())
    }

    /// Reset lcd display
    fn reset(&mut self) -> Result<(), ()> {
        self.command(0b0000_0010)?;

        Ok(())
    }

    /// On backlight of lcd display
    fn backlight(&mut self) -> Result<(), ()> {
        self.back_light_val = BACKLIGHT;
        self.display()?;

        Ok(())
    }

    /// Off backlight of lcd display
    fn no_backlight(&mut self) -> Result<(), ()> {
        self.back_light_val = NO_BACKLIGHT;
        self.display()?;

        Ok(())
    }

    /// On display
    fn display(&mut self) -> Result<(), ()> {
        self.display_control |= DISPLAY_ON;
        self.command(DISPLAY_CONTROL | self.display_control)?;

        Ok(())
    }

    /// Off display
    fn no_display(&mut self) -> Result<(), ()> {
        self.display_control &= !DISPLAY_ON;
        self.command(DISPLAY_CONTROL | self.display_control)?;

        Ok(())
    }

    /// Return cursor to start address (0, 0)
    fn home(&mut self) -> Result<(), ()> {
        self.command(RETURN_HOME)?;

        Ok(())
    }

    /// Set cursor to address (column, row)
    ///
    /// # Arguments
    ///
    /// * `col` - column number
    /// * `row` - row number
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
    /// let mut lcd = Lcd::new(i2c, 0x27, delay).build();
    ///
    /// lcd.init().unwrap();
    /// lcd.clear().unwrap();
    /// lcd.reset().unwrap();
    ///
    /// lcd.set_cursor(1, 8).unwrap();
    /// ```
    fn set_cursor(&mut self, col: u8, mut row: u8) -> Result<(), ()> {
        const ROW_OFFSETS: [u8; 4] = [0x00_u8, 0x40_u8, 0x14_u8, 0x54_u8];

        if row >= self.rows { row = self.rows - 1; }

        self.command(SET_DRAM_ADDR | (col + ROW_OFFSETS[row as usize]))?;

        Ok(())
    }

    /// Write char to lcd display
    ///
    /// # Arguments
    ///
    /// * `data` - char to write
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
    /// let mut lcd = Lcd::new(i2c, 0x27, delay).build();
    ///
    /// lcd.init().unwrap();
    /// lcd.clear().unwrap();
    /// lcd.reset().unwrap();
    ///
    /// lcd.write_char('A').unwrap();
    /// ```
    fn write_char(&mut self, data: char) -> Result<(), ()> {
        self.write_byte(data as u8)?;

        Ok(())
    }

    /// Write bytes to lcd display
    ///
    /// # Arguments
    ///
    /// * `data` - bytes to write
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
    /// let mut lcd = Lcd::new(i2c, 0x27, delay).build();
    ///
    /// lcd.init().unwrap();
    /// lcd.clear().unwrap();
    /// lcd.reset().unwrap();
    ///
    /// lcd.write_bytes(&['A' as u8, 'B' as u8, 'C' as u8])
    ///     .unwrap();
    /// ```
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), ()> {
        for &b in data {
            self.write_byte(b)?;
        }

        Ok(())
    }

    /// Write string to lcd display
    ///
    /// # Arguments
    ///
    /// * `data` - string to write
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
    /// let mut lcd = Lcd::new(i2c, 0x27, delay).build();
    ///
    /// lcd.init().unwrap();
    /// lcd.clear().unwrap();
    /// lcd.reset().unwrap();
    ///
    /// let string_to_write: &str = "Hello, World!";
    /// lcd.write_str(string_to_write).unwrap();
    /// ```
    fn write_str(&mut self, data: &str) -> Result<(), ()> {
        self.write_bytes(data.as_bytes())?;

        Ok(())
    }

    /// Create custom char for lcd display
    ///
    /// # Arguments
    ///
    /// * `location` - address of memory for use
    /// * `char_map` - map of bits in cell (8 rows by 5 bits)
    ///
    /// # Example
    ///
    /// ```
    /// use device_drivers::i2c::lcd::{LcdTrait, Lcd};
    /// let mut lcd = Lcd::new(i2c, 0x27, delay).build();
    ///
    /// lcd.init().unwrap();
    /// lcd.clear().unwrap();
    /// lcd.reset().unwrap();
    ///
    /// let mut custom_symbol: [u8; 8] = [0; u8];
    ///
    /// custom_symbol[0] = 0b00111;
    /// custom_symbol[1] = 0b01111;
    /// custom_symbol[2] = 0b11111;
    /// custom_symbol[3] = 0b11111;
    /// custom_symbol[4] = 0b11111;
    /// custom_symbol[5] = 0b11111;
    /// custom_symbol[6] = 0b11111;
    /// custom_symbol[7] = 0b11111;
    ///
    /// lcd.create_char(0, &custom_symbol).unwrap();
    ///
    /// ```
    fn create_char(&mut self, mut location: u8, char_map: &[u8; 8]) -> Result<(), ()> {
        location &= 0x07_u8;
        self.command(SET_CRAM_ADDR | (location << 3))?;

        for &ch in char_map {
            self.write_byte(ch)?;
        }

        Ok(())
    }

    /// Write to lcd custom char from memory by location
    ///
    /// # Arguments
    ///
    /// * `location` - location of char in memory lcd
    fn write_custom_char(&mut self, location: u8) -> Result<(), ()> {
        self.write_byte(location)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
}
