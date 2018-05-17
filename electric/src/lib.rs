extern crate utils;

pub mod battery;
pub mod circuit;
pub mod led;
pub mod component;
pub mod wire;
pub mod validator;
pub mod connector;

pub type ComponentStrong = std::rc::Rc<std::cell::RefCell<component::Component>>;
pub type ComponentWeak = std::rc::Weak<std::cell::RefCell<component::Component>>;

#[cfg(test)]
mod tests {
    use circuit::{Circuit, Lead};
    use led::Led;
    use battery::Battery;
    use component::Component;
    use validator::Error;
    use std::rc::Weak;

    #[test]
    fn it_works() {
        let mut circuit = Circuit::default();
        let led = circuit.add(Component::Led(Led {
            min_voltage: 1.75,
            max_voltage: 2.25,
            current_drop: 0.020,
            .. Led::default()
        }));
        let battery = circuit.add(Component::Battery(Battery {
            voltage: 5f32,
            .. Battery::default()
        }));
        let _wire_1 = circuit.add_wire_between((&battery, Lead::Positive), (&led, Lead::Input));
        let _wire_2 = circuit.add_wire_between((&led, Lead::Output), (&battery, Lead::Negative));

        let errors = circuit.validate().errors;
        assert!(errors.len() > 0);
        let error = errors.iter().find(|e| e.is_high_voltage_error());
        assert!(error.is_some());

        let error = match error {
            Some(Error::HighVoltage(e)) => e,
            _ => unreachable!(),
        };
        let error_component = Weak::upgrade(&error.component).unwrap();
        let led_component = Weak::upgrade(&led).unwrap();
        assert_eq!(error_component.as_ptr(), led_component.as_ptr());
        assert_eq!(5.0, error.input_voltage);
        assert_eq!((1.75, 2.25), error.expected_voltage);
    }
}
