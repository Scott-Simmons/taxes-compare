#[derive(Debug, PartialEq)]
pub enum TaxError {
    NegativeIncome,
    IncomeOutOfBounds,
}
