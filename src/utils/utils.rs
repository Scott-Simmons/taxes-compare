use crate::IncomeTaxPoint;

/// Utility function for generating a range of income points to interpolate.
pub fn generate_range(start: f32, stop: f32, step: f32) -> Vec<f32> {
    let mut values = Vec::new();
    let mut current = start;
    while current <= stop {
        values.push(current);
        current += step;
    }
    values
}

pub fn income_points_are_approx_eq(
    point1: IncomeTaxPoint,
    point2: IncomeTaxPoint,
    tol: f32,
) -> bool {
    let x1 = point1.income;
    let x2 = point2.income;
    let y1 = point1.income_tax_amount;
    let y2 = point2.income_tax_amount;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() < tol
}
