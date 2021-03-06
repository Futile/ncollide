use num::Float;
use na;
use math::{Point, Vect};
use entities::shape::Ball;

/// Distance between balls.
#[inline]
pub fn ball_against_ball<P>(center1: &P, b1: &Ball<<P::Vect as Vect>::Scalar>,
                            center2: &P, b2: &Ball<<P::Vect as Vect>::Scalar>)
                            -> <P::Vect as Vect>::Scalar
    where P: Point {
    let r1         = b1.radius();
    let r2         = b2.radius();
    let delta_pos  = *center2 - *center1;
    let sqdist     = na::sqnorm(&delta_pos);
    let sum_radius = r1 + r2;

    if sqdist <= sum_radius * sum_radius {
        na::zero()
    }
    else {
        sqdist.sqrt() - sum_radius
    }
}
