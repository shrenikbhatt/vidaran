pub trait Round {
    fn to_two_decimals(self) -> f32;
}

impl Round for f32 {
    fn to_two_decimals(self) -> f32 {
        ((self * 100_f32) as f32).round() / 100_f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_two_decimals_rounds_up() {
        let num: f32 = 1.235210983901;
        assert_eq!(num.to_two_decimals(), 1.24)
    }

    #[test]
    fn test_to_two_decimals_rounds_down() {
        let num: f32 = 1.2345210983901;
        assert_eq!(num.to_two_decimals(), 1.23)
    }
}
