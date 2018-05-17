use ComponentWeak;

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    Unknown,
    HighVoltage(HighVoltage),
}

impl Error {
    pub fn is_high_voltage_error(&self) -> bool {
        match *self {
            Error::HighVoltage(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct HighVoltage {
    pub component: ComponentWeak,
    pub input_voltage: f32,
    pub expected_voltage: (f32, f32),
}
