const AMOUNT_STORAGE_PRECISION: f64 = 10_000.0;

/// Super simple structure to store and modify monetary values as an integer.
/// Provides 4 decimal places of accuracy.
#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Amount(i64);

impl Amount {
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }
}

impl From<f64> for Amount {
    fn from(value: f64) -> Self {
        let int_val: i64 = (value * AMOUNT_STORAGE_PRECISION).round() as i64;
        Self(int_val)
    }
}

impl From<Amount> for f64 {
    fn from(value: Amount) -> Self {
        value.0 as f64 / AMOUNT_STORAGE_PRECISION
    }
}

#[test]
fn test_amount_conversion() {
    assert_eq!(Amount::from(1.0).0, 10000);
    assert_eq!(Amount::from(1.5241).0, 15241);
}

#[test]
fn test_amount_conversion_four_places_precision() {
    assert_eq!(Amount::from(12345.52413).0, 123455241);
    assert_eq!(Amount::from(-0.52408).0, -5241);
}

#[test]
fn test_amount_reverse_conversion() {
    assert_eq!(f64::from(Amount(10000)), 1.0);
    assert_eq!(f64::from(Amount(15241)), 1.5241);
    assert_eq!(f64::from(Amount(123455241)), 12345.5241);
    assert_eq!(f64::from(Amount(-5241)), -0.5241);
}

impl std::ops::Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        Amount(self.0 + rhs.0)
    }
}

impl std::ops::Sub<Amount> for Amount {
    type Output = Amount;

    fn sub(self, rhs: Amount) -> Self::Output {
        Amount(self.0 - rhs.0)
    }
}

#[test]
fn test_amount_addition_subtraction() {
    assert_eq!(Amount(10000) + Amount(15241), Amount(25241));
    assert_eq!(Amount(10000) - Amount(15241), Amount(-5241));
}
