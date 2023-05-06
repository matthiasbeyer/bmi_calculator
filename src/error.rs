#[derive(Debug, PartialEq)]
pub enum BmiError {
    HeightCannotBeZeroOrSmaller,
    WeightCannotBeZeroOrSmaller,
}
