use wire::Wire;
use super::State;
use validator::ValidationResult;
use component::Component;
use ComponentWeak;
use circuit::Lead;
use std::rc::Weak;

#[derive(Debug)]
pub struct WireState<'a> {
    pub wire: &'a Wire,
    pub voltage: f32,
}

impl<'a> WireState<'a> {
    pub fn from(wire: &Wire) -> WireState {
        WireState {
            wire,
            voltage: 0f32,
        }
    }
    
    fn update(&mut self, component: &ComponentWeak, lead: Option<Lead>) -> bool {
        if let Some(upgraded) = Weak::upgrade(component) {
            let borrowed: &Component = &*upgraded.borrow();
            if let Component::Battery(ref batt) = borrowed {
                if lead == Some(Lead::Positive) {
                    if batt.voltage > self.voltage {
                        println!("Wire voltage went from {} to {}", self.voltage, batt.voltage);
                        self.voltage = batt.voltage;
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl<'a> State<'a> for WireState<'a> {
    fn update(&mut self, _others: &Vec<&Box<State + 'a>>) -> bool {
        let mut did_update = false;
        if let Some((ref component, lead)) = self.wire.connect_from {
            did_update = self.update(component, lead) || did_update;
        }
        if let Some((ref component, lead)) = self.wire.connect_to {
            did_update = self.update(component, lead) || did_update;
        }
        did_update
    }

    fn finalize(&self, _result: &mut ValidationResult){}

    fn is(&self, component: &Component) -> bool {
        match component {
            &Component::Wire(ref w) => w as *const _ == self.wire as *const _,
            _ => false
        }
    }

    fn voltage(&self, _: Option<Lead>) -> Option<f32> {
        Some(self.voltage)
    }
}
