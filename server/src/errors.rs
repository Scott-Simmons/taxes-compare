#[derive(Debug, PartialEq)]
pub enum TaxError {
    NegativeIncome(f32),
    IncomeOutOfBounds { income: f32, bounds: (f32, f32) },
}

impl std::fmt::Display for TaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxError::NegativeIncome(income) => {
                write!(f, "Negative income encountered: {}", income)
            }
            TaxError::IncomeOutOfBounds { income, bounds } => {
                write!(f, "Income {} is out of bounds: {:?}", income, bounds)
            }
        }
    }
}

impl std::error::Error for TaxError {}
