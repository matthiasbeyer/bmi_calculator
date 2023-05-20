#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BodyMassIndex {
    value: f64,
}

impl BodyMassIndex {
    pub fn new(bmi: f64) -> BodyMassIndex {
        BodyMassIndex { value: bmi }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
