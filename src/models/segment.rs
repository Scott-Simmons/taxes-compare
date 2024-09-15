use crate::IncomeTaxKnot;
use crate::IncomeTaxPoint;
use std::cmp::Ordering;

/// A line segment characterised by two points
#[derive(Clone, Debug, PartialEq)]
pub struct LinearPiecewiseSegment {
    /// Two knot points characterise a segment of a linear piecewise function
    /// https://en.wikipedia.org/wiki/Line_segment
    pub left_point: IncomeTaxKnot, // fix up later
    pub right_point: IncomeTaxKnot, // fix up later
                                    // some kind of assert so that left point x value always less than right point x value
}
/// Linearly interpolate a line segment at an income value, to get a taxation value.
impl LinearPiecewiseSegment {
    pub fn linear_interpolation(&self, income: f32) -> Option<f32> {
        if income < f32::min(self.left_point.income_limit, self.right_point.income_limit)
            || income > f32::max(self.right_point.income_limit, self.left_point.income_limit)
        {
            return None;
        }
        Some(
            self.left_point.income_tax_amount
                + ((self.right_point.income_tax_amount - self.left_point.income_tax_amount)
                    * (income - self.left_point.income_limit))
                    / (self.right_point.income_limit - self.left_point.income_limit),
        )
    }
    /// Gets line segments into a form parameterised as l = a * t(b - a) where a and b are
    /// points in R^2 and t \in [0, 1]
    /// Then equate l1 = l2 and solve under those constraints
    /// The maths yields t1(b1 - a1) + t2(a2 - b2) = (a2 - a1)
    /// This can then be brought into the form At = c
    /// Three types of solution: Independent (one solution), Dependent (infininity many -
    /// return the whole line, inconsistent - return None). Rank(A) = 2, Rank(A) < 2, Rank(A) >
    /// 2
    /// See: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    /// If the segments are colinear it will return None
    pub fn compute_intersection(
        &self,
        segment_to_intersect: &LinearPiecewiseSegment,
    ) -> Option<IncomeTaxPoint> {
        let x1 = self.left_point.income_limit;
        let x2 = self.right_point.income_limit;
        let y1 = self.left_point.income_tax_amount;
        let y2 = self.right_point.income_tax_amount;
        let q1 = segment_to_intersect.left_point.income_limit;
        let q2 = segment_to_intersect.right_point.income_limit;
        let r1 = segment_to_intersect.left_point.income_tax_amount;
        let r2 = segment_to_intersect.right_point.income_tax_amount;
        //
        // https://docs.rs/nalgebra/latest/nalgebra/macro.matrix.html
        let matrix_lhs = nalgebra::matrix![
            x2 - x1, q1 - q2;
            y2 - y1, r1 - r2
        ];
        let row_size: usize = 2;
        let tol: f32 = 0.00000001;
        match matrix_lhs.rank(tol).cmp(&row_size) {
            Ordering::Less => None,    // linearly dependent - coincident
            Ordering::Greater => None, // overdetermined, parallel
            Ordering::Equal => {
                // solve Ax=b
                let b_rhs = nalgebra::matrix![q1 - x1; r1 - y1];
                match matrix_lhs.lu().solve(&b_rhs) {
                    None => None,
                    Some(solution) => {
                        // Verify solution satisfies t \in [0, 1]
                        let (t1, t2) = (solution[(0, 0)], solution[(1, 0)]);
                        if (0.0 <= t1 && t1 <= 1.0) && (0.0 <= t2 && t2 <= 1.0) {
                            Some(IncomeTaxPoint {
                                income_tax_amount: y1 + t1 * (y2 - y1),
                                income: x1 + t1 * (x2 - x1),
                            })
                        } else {
                            None // No intersection within the segment range.
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IncomeTaxKnot, IncomeTaxPoint, LinearPiecewiseSegment};
    #[test]
    fn test_linear_interpolation() {
        let segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 3.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 4.0,
                income_tax_amount: 6.0,
            },
        };
        let valid_result = segment.linear_interpolation(4.5);
        assert_eq!(valid_result, Some(4.5));

        let invalid_result = segment.linear_interpolation(5.1);
        assert_eq!(invalid_result, None);

        let invalid_result_2 = segment.linear_interpolation(3.9);
        assert_eq!(invalid_result_2, None);
    }

    #[test]
    fn test_get_breakeven_point() {
        // https://www.desmos.com/calculato
        let test_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 10.0,
                income_tax_amount: 0.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 10.0,
            },
        };
        let interecting_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 10.0,
                income_tax_amount: 10.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
        };

        let result = test_segment.compute_intersection(&interecting_segment);
        assert_eq!(
            result.unwrap(),
            IncomeTaxPoint {
                income_tax_amount: 5.0,
                income: 5.0
            }
        );

        let barely_interecting_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 5.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
        };
        let result = test_segment.compute_intersection(&barely_interecting_segment);
        assert_eq!(
            result.unwrap(),
            IncomeTaxPoint {
                income_tax_amount: 5.0,
                income: 5.0
            }
        );

        let non_interecting_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 4.0,
                income_tax_amount: 4.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
        };
        let result = test_segment.compute_intersection(&non_interecting_segment);
        assert!(result.is_none());

        let result = test_segment.compute_intersection(&test_segment);
        assert!(result.is_none());

        let parallel_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 5.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 0.0,
            },
        };
        let result = parallel_segment.compute_intersection(&test_segment);
        assert!(result.is_none());

        // Need to also test process segments that doesn't line up in x
    }
}
