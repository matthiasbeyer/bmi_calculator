#[cfg(test)]
mod test {
    use crate::{calculate_bmi, BmiError, Height, Weight};

    use float_eq::assert_float_eq;

    #[test]
    fn test_calculate_bmi() {
        let result = calculate_bmi(Weight(69.0), Height(1.69)).unwrap();
        assert_float_eq!(result.value(), 24.15, abs <= 0.01);
    }

    #[test]
    fn test_calculate_bmi_broken() {
        let opt = calculate_bmi(Weight(69.0), Height(-0.0));
        assert!(opt.is_err());
        let err = opt.unwrap_err();
        assert_eq!(err, BmiError::HeightCannotBeZeroOrSmaller);
    }

    #[test]
    fn test_calculate_bmi_broken_weigth() {
        let res = calculate_bmi(Weight(-0.0), Height(172.0));
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, BmiError::WeightCannotBeZeroOrSmaller);
    }
}
