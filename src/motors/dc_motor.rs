use crate::motors::Motor;
use driver::AttributeResult;

pub const RUN_FOREVER: &str = "run-forever";
pub const RUN_TIMED: &str = "run-timed";
pub const RUN_DIRECT: &str = "run-direct";
pub const STOP: &str = "stop";

pub const POLARITY_NORMAL: &str = "normal";
pub const POLARITY: &str = "reversed";

pub const STATE_RUNNING: &str = "running";
pub const STATE_RAMPING: &str = "ramping";

pub const STOP_ACTION_COAST: &str = "coast";
pub const STOP_ACTION_BRAKE: &str = "brake";

pub trait DcMotor: Motor {
    fn get_address(&self) -> AttributeResult<String> {
        self.get_attribute("address").get()
    }
    fn set_command(&self, command: &str) -> AttributeResult<()> {
        self.get_attribute("command").set_str_slice(command)
    }
    fn get_commands(&self) -> AttributeResult<Vec<String>> {
        self.get_attribute("commands").get_vec()
    }

    fn get_driver_name(&self) -> AttributeResult<String> {
        self.get_attribute("driver_name").get()
    }

    fn get_duty_cycle(&self) -> AttributeResult<i32> {
        self.get_attribute("duty_cycle").get()
    }
    fn get_duty_cycle_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("duty_cycle_sp").get()
    }
    fn set_duty_cycle_sp(&self, duty_cycle_sp: i32) -> AttributeResult<()> {
        self.get_attribute("duty_cycle_sp").set(duty_cycle_sp)
    }

    fn get_polarity(&self) -> AttributeResult<String> {
        self.get_attribute("polarity").get()
    }
    fn set_polarity(&self, polarity: &str) -> AttributeResult<()> {
        self.get_attribute("polarity").set_str_slice(polarity)
    }

    fn get_ramp_up_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("ramp_up_sp").get()
    }
    fn set_ramp_up_sp(&self, ramp_up_sp: i32) -> AttributeResult<()> {
        self.get_attribute("ramp_up_sp").set(ramp_up_sp)
    }

    fn get_ramp_down_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("ramp_down_sp").get()
    }
    fn set_ramp_down_sp(&self, ramp_down_sp: i32) -> AttributeResult<()> {
        self.get_attribute("ramp_down_sp").set(ramp_down_sp)
    }

    fn get_state(&self) -> AttributeResult<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    fn get_stop_action(&self) -> AttributeResult<String> {
        self.get_attribute("stop_action").get()
    }
    fn set_stop_action(&self, stop_action: &str) -> AttributeResult<()> {
        self.get_attribute("stop_action").set_str_slice(stop_action)
    }

    fn get_time_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("time_sp").get()
    }
    fn set_time_sp(&self, time_sp: i32) -> AttributeResult<()> {
        self.get_attribute("time_sp").set(time_sp)
    }

    fn is_running(&self) -> AttributeResult<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
    }
    fn is_ramping(&self) -> AttributeResult<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_RAMPING))
    }
}
