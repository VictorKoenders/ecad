use battery::Battery;
use super::State;
use validator::ValidationResult;
use component::Component;
use circuit::Lead;

#[derive(Debug)]
pub struct BatteryState<'a> {
    pub battery: &'a Battery,
}

impl<'a> BatteryState<'a> {
    pub fn from(battery: &Battery) -> BatteryState {
        BatteryState { battery }
    }
}
impl<'a> State<'a> for BatteryState<'a> {
    fn update(&mut self, _others: &Vec<&Box<State + 'a>>) -> bool {
        false
    }

    fn finalize(&self, _result: &mut ValidationResult) {}

    fn is(&self, component: &Component) -> bool {
        match component {
            &Component::Battery(ref b) => b as *const _ == self.battery as *const _,
            _ => false
        }
    }

    fn voltage(&self, lead: Option<Lead>) -> Option<f32> {
        match lead {
            Some(Lead::Positive) => Some(self.battery.voltage),
            Some(Lead::Negative) => Some(0f32),
            _ => None
        }
    }
}
