//! LEGO EV3 color sensor.

use std::{str::FromStr, sync::{Arc, RwLock}};

use super::{Sensor, SensorPort};
use crate::{sensor_mode, Attribute, Device, Driver, Ev3Error, Ev3Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinDataFormat {
/// Unsigned 8-bit integer (byte)
U8,
/// Signed 8-bit integer (sbyte)
S8,
/// Unsigned 16-bit integer (ushort)
U16,
/// Signed 16-bit integer (short)
S16,
/// Signed 16-bit integer, big endian
S16Be,
/// Signed 32-bit integer (int)
S32,
/// IEEE 754 32-bit floating point (float)
Float,
}

impl BinDataFormat {
    pub fn size(&self) -> u8 {
        match self {
            BinDataFormat::U8 => 1,
            BinDataFormat::S8 => 1,
            BinDataFormat::U16 => 2,
            BinDataFormat::S16 => 2,
            BinDataFormat::S16Be => 2,
            BinDataFormat::S32 => 4,
            BinDataFormat::Float => 4,
        }
    }
}

impl FromStr for BinDataFormat {
    type Err = Ev3Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "u8" => Ok(BinDataFormat::U8),
            "s8" => Ok(BinDataFormat::S8),
            "u16" => Ok(BinDataFormat::U16),
            "s16" => Ok(BinDataFormat::S16),
            "s16_be" => Ok(BinDataFormat::S16Be),
            "s32" => Ok(BinDataFormat::S32),
            "float" => Ok(BinDataFormat::Float),
            _ => panic!("Invalid bin data format")
        }
    }
}



/// LEGO EV3 color sensor.
#[derive(Debug, Clone, Device, Sensor)]
pub struct ColorSensor {
    driver: Driver,
    num_values: Arc<RwLock<Option<u8>>>,
    bin_format: Arc<RwLock<Option<BinDataFormat>>>,
}

impl ColorSensor {
    fn new(driver: Driver) -> Self {
        Self { driver, num_values: Arc::new(RwLock::new(None)), bin_format: Arc::new(RwLock::new(None)) }
    }

    findable!(
        "lego-sensor",
        ["lego-ev3-color"],
        SensorPort,
        "ColorSensor",
        "in"
    );

    sensor_mode!(
        "COL-REFLECT",
        MODE_COL_REFLECT,
        "Reflected light - sets LED color to red",
        set_mode_col_reflect,
        is_mode_col_reflect
    );
    sensor_mode!(
        "COL-AMBIENT",
        MODE_COL_AMBIENT,
        "Ambient light - sets LED color to blue (dimly lit)",
        set_mode_col_ambient,
        is_mode_col_ambient
    );
    sensor_mode!(
        "COL-COLOR",
        MODE_COL_COLOR,
        "Color - sets LED color to white (all LEDs rapidly cycling)",
        set_mode_col_color,
        is_mode_col_color
    );
    sensor_mode!(
        "REF-RAW",
        MODE_REF_RAW,
        "Raw Reflected - sets LED color to red",
        set_mode_ref_raw,
        is_mode_ref_raw
    );
    sensor_mode!(
        "RGB-RAW",
        MODE_RGB_RAW,
        "Raw Color Components - sets LED color to white (all LEDs rapidly cycling)",
        set_mode_rgb_raw,
        is_mode_rgb_raw
    );
    sensor_mode!(
        "COL-CAL",
        MODE_COL_CAL,
        "Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous",
        set_mode_col_cal,
        is_mode_col_cal
    );

    /// Get the color value for the modes COL-REFLECT, COL-AMBIENT, COL-COLOR and REF-RAW.
    pub fn get_color(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// Red component of the detected color, in the range 0-1020.
    pub fn get_red(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// Green component of the detected color, in the range 0-1020.
    pub fn get_green(&self) -> Ev3Result<i32> {
        self.get_value1()
    }

    /// Blue component of the detected color, in the range 0-1020.
    pub fn get_blue(&self) -> Ev3Result<i32> {
        self.get_value2()
    }

    /// Red, green and blue components of the detected color, each in the range 0-1020
    pub fn get_rgb(&self) -> Ev3Result<(i32, i32, i32)> {
        let red = self.get_red()?;
        let green = self.get_green()?;
        let blue = self.get_blue()?;

        Ok((red, green, blue))
    }


    /// Returns the unscaled raw values in the `value<N>` attributes as raw byte
    /// array. Use `bin_data_format`, `num_values` and the individual sensor
    /// documentation to determine how to interpret the data.
    pub fn get_bin_data(&self) -> Ev3Result<(i16, i16, i16)> {
        let outer = self.num_values.clone();
        let num_values = outer.read().unwrap();
        let num_values = if num_values.is_some() {
                num_values.unwrap()
        } else {
            drop(num_values);
            let num: u8 = self.get_attribute("num_values").get()?;

            let mut writer = outer.write().unwrap();
            writer.replace(num);

            num
        };

        let outer = self.bin_format.clone();
        let bin_format = outer.read().unwrap();
        let bin_format = if bin_format.is_some() {
            bin_format.unwrap()
        } else {
            drop(bin_format);
            let format: BinDataFormat = self.get_attribute("bin_data_format").get()?;
          
            let mut writer = outer.write().unwrap();
            writer.replace(format);

            format
        };

        if bin_format != BinDataFormat::S16 {
            panic!("get_bin_data is not supported for this color sensor")
        }

        let data = self.get_attribute("bin_data").get_raw_data()?;

        let colors: Vec<i16> = data[0..8]
            .chunks_exact(2)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]))
            .collect();
   
        Ok((colors[0], colors[1], colors[2]))
    }
}
