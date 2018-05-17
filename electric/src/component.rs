use wire::Wire;
use battery::Battery;
use led::Led;
use circuit::Lead;
use utils::point::Point;
use ComponentWeak;

#[derive(Debug)]
pub enum Component {
    Wire(Wire),
    Led(Led),
    Battery(Battery),
}

impl Component {
    pub fn get_lead_connector(&self, lead: Lead) -> Option<Point> {
        match *self {
            Component::Wire(_) => None,
            Component::Led(ref led) => led.get_lead_connector(lead),
            Component::Battery(ref batt) => batt.get_lead_connector(lead),
        }
    }

    pub fn set_lead_connector(&mut self, lead: Lead, component: ComponentWeak, other_lead: Option<Lead>) {
        match *self {
            Component::Wire(_) => {},
            Component::Led(ref mut led) => led.set_lead_connector(lead, component, other_lead),
            Component::Battery(ref mut batt) => batt.set_lead_connector(lead, component, other_lead),
        }
    }

    pub fn is_wire(&self) -> bool {
        match *self {
            Component::Wire(_) => true,
            _ => false,
        }
    }

    pub fn as_wire(&self) -> Option<&Wire> {
        match *self {
            Component::Wire(ref wire) => Some(wire),
            _ => None,
        }
    }

    pub fn is_led(&self) -> bool {
        match *self {
            Component::Led(_) => true,
            _ => false,
        }
    }

    pub fn as_led(&self) -> Option<&Led> {
        match *self {
            Component::Led(ref led) => Some(led),
            _ => None,
        }
    }
}
