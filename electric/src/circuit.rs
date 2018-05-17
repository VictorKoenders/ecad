use std::rc::{Weak, Rc};
use component::Component;
use wire::Wire;
use validator::{validate, ValidationResult};
use std::cell::RefCell;
use {ComponentWeak, ComponentStrong};

#[derive(Default, Debug)]
pub struct Circuit {
    pub components: Vec<ComponentStrong>,
}

impl Circuit {
    pub fn add(&mut self, component: Component) -> Weak<RefCell<Component>> {
        let component = Rc::new(RefCell::new(component));
        let weak = Rc::downgrade(&component);
        self.components.push(component);
        weak
    }

    pub fn add_wire_between(&mut self, from: (&ComponentWeak, Lead), to: (&ComponentWeak, Lead)) -> Option<ComponentWeak> {
        let component_from = from.0.upgrade()?;
        let component_to = to.0.upgrade()?;

        let connection_from = component_from.borrow().get_lead_connector(from.1)?;
        let connection_to = component_to.borrow().get_lead_connector(to.1)?;

        let wire = Rc::new(RefCell::new(Component::Wire(Wire {
            from: connection_from,
            to: connection_to,

            connect_from: Some((from.0.clone(), Some(from.1))),
            connect_to: Some((to.0.clone(), Some(to.1))),
        })));

        let weak = Rc::downgrade(&wire);

        component_from.borrow_mut().set_lead_connector(from.1, weak.clone(), Some(to.1));
        component_to.borrow_mut().set_lead_connector(to.1, weak.clone(), Some(from.1));

        self.components.push(wire);

        Some(weak)
    }

    pub fn validate(&self) -> ValidationResult {
        validate(&self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lead {
    Positive,
    Negative,
    Input,
    Output
}
