use utils::point::Point;
use ComponentWeak;
use circuit::Lead;
use connector::Connector;

#[derive(Default, Debug)]
pub struct Led {
    pub position: Point,
    pub min_voltage: f32,
    pub max_voltage: f32,
    pub current_drop: f32,

    pub connectors: Vec<Connector>,
}

impl Led {
    pub fn get_lead_connector(&self, lead: Lead) -> Option<Point> {
        match lead {
            Lead::Input => Some(self.position + Point::new(1f32, 0f32)),
            Lead::Output => Some(self.position - Point::new(1f32, 0f32)),
            _ => None
        }
    }
    
    pub fn set_lead_connector(&mut self, lead: Lead, component: ComponentWeak, other_lead: Option<Lead>) {
        if lead != Lead::Input  && lead != Lead::Output { return; }
        for connector in &mut self.connectors {
            if connector.own_lead == Some(lead) {
                connector.component = component;
                connector.other_lead = other_lead;
                return;
            }
        }
        self.connectors.push(Connector {
            component,
            other_lead,
            own_lead: Some(lead),
        })
    }
}
