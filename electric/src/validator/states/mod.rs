mod led_state;
mod wire_state;
mod battery_state;

pub use self::led_state::LedState;
pub use self::wire_state::WireState;
pub use self::battery_state::BatteryState;

use component::Component;
use super::ValidationResult;
use circuit::Lead;

pub trait State<'a> {
    fn update(&mut self, others: &Vec<&Box<State + 'a>>) -> bool;
    fn finalize(&self, result: &mut ValidationResult);
    fn is(&self, component: &Component) -> bool;
    fn voltage(&self, lead: Option<Lead>) -> Option<f32>;
}

