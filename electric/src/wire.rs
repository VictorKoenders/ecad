use utils::point::Point;
use ComponentWeak;
use circuit::Lead;

#[derive(Default, Debug)]
pub struct Wire {
    pub from: Point,
    pub to: Point,

    pub connect_from: Option<(ComponentWeak, Option<Lead>)>,
    pub connect_to: Option<(ComponentWeak, Option<Lead>)>,
}
