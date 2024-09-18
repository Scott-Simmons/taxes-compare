use serde::Deserialize;

/// A point characterised by tax amount at given income, which is also denoted as a knot point
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IncomeTaxKnot {
    /// Income tax amount f(x) for a given maximimum income level x
    income_tax_amount: f32,
    /// The income threshold at which the knot acts as the boundry point
    income_limit: f32,
}

impl IncomeTaxKnot {
    /// Example: IncomeTaxKnot::new(x,y)
    pub fn new(income_limit: f32, income_tax_amount: f32) -> Self {
        Self {
            income_limit,
            income_tax_amount,
        }
    }

    pub fn income_tax_amount(&self) -> f32 {
        self.income_tax_amount
    }
    pub fn income_limit(&self) -> f32 {
        self.income_limit
    }
}

/// A point characterised by tax amount at a given income
#[derive(Debug, PartialEq, Clone)]
pub struct IncomeTaxPoint {
    /// Income tax amount f(x) for given level of income x
    income_tax_amount: f32,
    /// Level of income x
    income: f32,
}

impl IncomeTaxPoint {
    /// Example: IncomeTaxPoint::new(x,y)
    pub fn new(income: f32, income_tax_amount: f32) -> Self {
        Self {
            income,
            income_tax_amount,
        }
    }

    pub fn income_tax_amount(&self) -> f32 {
        self.income_tax_amount
    }
    pub fn income(&self) -> f32 {
        self.income
    }
}
