use led::Led;
use super::State;
use validator::ValidationResult;
use component::Component;
use std::rc::{Rc, Weak};
use circuit::Lead;
use validator::error::{Error, HighVoltage};
use ComponentStrong;

#[derive(Debug)]
pub struct LedState<'a> {
    pub led: &'a Led,
    pub voltage: f32,
    pub component: ComponentStrong,
}

impl<'a> LedState<'a> {
    pub fn from(led: &Led, component: ComponentStrong) -> LedState {
        LedState { led, voltage: 0f32, component }
    }
}
impl<'a> State<'a> for LedState<'a> {
    fn update(&mut self, others: &Vec<&Box<State + 'a>>) -> bool {
        for connector in &self.led.connectors {
            if let Some(component) = Weak::upgrade(&connector.component) {
                let borrowed = &*component.borrow();
                if let Some(state) = others.iter().find(|s| s.is(borrowed)) {
                    if let Some(voltage) = state.voltage(connector.other_lead) {
                        if voltage > self.voltage {
                            self.voltage = voltage;
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn finalize(&self, result: &mut ValidationResult) {
        if self.voltage > self.led.max_voltage {
            println!("Too much voltage on LED! Expected between {} and {}, got {}", self.led.min_voltage, self.led.max_voltage, self.voltage);
            println!("Suggestion: Add a resistor between {} and {} ohm",
                ((self.voltage - self.led.max_voltage) / self.led.current_drop).ceil(),
                ((self.voltage - self.led.min_voltage) / self.led.current_drop).floor(),
            );
            result.errors.push(Error::HighVoltage(HighVoltage {
                component: Rc::downgrade(&self.component),
                input_voltage: self.voltage,
                expected_voltage: (self.led.min_voltage, self.led.max_voltage),
            }));
        }
    }

    fn is(&self, component: &Component) -> bool {
        match component {
            &Component::Led(ref l) => l as *const _ == self.led as *const _,
            _ => false
        }
    }

    fn voltage(&self, lead: Option<Lead>) -> Option<f32> {
        match lead {
            Some(Lead::Output) => Some(self.voltage),
            _ => None,
        }
    }
}
