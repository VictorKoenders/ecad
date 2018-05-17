use ComponentWeak;
use circuit::Lead;

#[derive(Debug)]
pub struct Connector {
    pub component: ComponentWeak,
    pub own_lead: Option<Lead>,
    pub other_lead: Option<Lead>,
}