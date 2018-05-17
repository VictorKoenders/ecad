mod result;
mod error;
mod states;

use ComponentStrong;
use circuit::Circuit;
pub use self::result::ValidationResult;
pub use self::error::Error;
use component::Component;
use self::states::{State, BatteryState, LedState, WireState};
use std::cell::Ref;

fn wrap<'a>(component: &'a (ComponentStrong, Ref<'a, Component>)) -> Box<State<'a> + 'a> {
    match &*(component.1) {
        &Component::Battery(ref batt) => Box::new(BatteryState::from(batt)),
        &Component::Led(ref led) => Box::new(LedState::from(led, component.0.clone())),
        &Component::Wire(ref wire) => Box::new(WireState::from(wire)),
    }
}
pub fn validate(circuit: &Circuit) -> ValidationResult {
    let borrowed: Vec<_> = circuit.components.iter().map(|i| (i.clone(), i.borrow())).collect();
    let mut states: Vec<Box<State>> = borrowed.iter().map(|b| wrap(b)).collect::<Vec<_>>();

    loop {
        let mut changes = 0;
        for i in 0..states.len() {
            let (before, remaining) = states.split_at_mut(i);
            let (current, after) = remaining.split_at_mut(1);

            assert_eq!(1, current.len());
            let items = before.iter().chain(after.iter()).collect::<Vec<_>>();
            if current[0].update(&items) {
                changes += 1;
            }
        }

        if changes == 0 {
            break;
        }
    }

    let mut result = ValidationResult::default();
    for state in states {
        state.finalize(&mut result);
    }

    result
}
