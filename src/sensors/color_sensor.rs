use crate::sensors::{Sensor, SensorPort};
use core::Device;
use driver::Attribute;
use driver::AttributeResult;
use driver::Driver;

/// Reflected light - sets LED color to red
pub const MODE_COL_REFLECT: &str = "COL-REFLECT";

/// Ambient light - sets LED color to blue (dimly lit)
pub const MODE_COL_AMBIENT: &str = "COL-AMBIENT";

/// Color - sets LED color to white (all LEDs rapidly cycling)
pub const MODE_COL_COLOR: &str = "COL-COLOR";

/// Raw Reflected - sets LED color to red
pub const MODE_REF_RAW: &str = "REF-RAW";

/// Raw Color Components - sets LED color to white (all LEDs rapidly cycling)
pub const MODE_RGB_RAW: &str = "RGB-RAW";

/// Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous
pub const MODE_COL_CAL: &str = "COL-CAL";

#[derive(Debug, Clone)]
pub struct ColorSensor {
    driver: Driver,
}

impl Sensor for ColorSensor {}

impl Device for ColorSensor {
    fn get_attribute(&self, name: &str) -> Attribute {
        self.driver.get_attribute(name)
    }
}

impl ColorSensor {
    /// Try to get a `ColorSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Option<ColorSensor> {
        if let Some(name) =
            Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-color")
        {
            return Some(ColorSensor {
                driver: Driver::new("lego-sensor", &name),
            });
        }

        None
    }

    /// Try to find a `ColorSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<ColorSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-color") {
            return Some(ColorSensor {
                driver: Driver::new("lego-sensor", &name),
            });
        }

        None
    }

    pub fn set_mode_col_reflect(&self) -> AttributeResult<()> {
        self.set_mode(MODE_COL_REFLECT)
    }

    pub fn set_mode_col_ambient(&self) -> AttributeResult<()> {
        self.set_mode(MODE_COL_AMBIENT)
    }

    pub fn set_mode_col_color(&self) -> AttributeResult<()> {
        self.set_mode(MODE_COL_COLOR)
    }

    pub fn set_mode_ref_raw(&self) -> AttributeResult<()> {
        self.set_mode(MODE_REF_RAW)
    }

    pub fn set_mode_rgb_raw(&self) -> AttributeResult<()> {
        self.set_mode(MODE_RGB_RAW)
    }

    pub fn set_mode_col_cal(&self) -> AttributeResult<()> {
        self.set_mode(MODE_COL_CAL)
    }

    pub fn get_red(&self) -> AttributeResult<i32> {
        self.get_value0()
    }
    pub fn get_green(&self) -> AttributeResult<i32> {
        self.get_value1()
    }
    pub fn get_blue(&self) -> AttributeResult<i32> {
        self.get_value2()
    }

    pub fn get_rgb(&self) -> AttributeResult<(i32, i32, i32)> {
        let red = self.get_red()?;
        let green = self.get_green()?;
        let blue = self.get_blue()?;

        Ok((red, green, blue))
    }
}