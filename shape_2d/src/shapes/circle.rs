use std::{ops::{Add, Sub, Mul, Div}, f64::consts::PI};

use shape_1d::Scalar;
use crate::Shape2d;

pub struct Circle<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
  radius : Scalar<T>
}

impl<T> Shape2d<T> for Circle<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn get_area(&self) -> Scalar<T> {
        let radius = *self.get_radius();
        return radius * radius * Scalar::new(PI);
    }
} 

impl<T> Circle<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn new(radius : Scalar<T>) -> Self {
        return Self {
            radius: radius,
        };
    }

    fn get_radius(&self) -> &Scalar<T> {
        return &self.radius;
    }
}

#[cfg(test)]
mod tests {
    use shape_1d::Scalar;
    use crate::{shapes::circle::Circle, Shape2d};

    #[test]
    fn test_circle_properties() {
        let circle_5 = Circle::new(Scalar::new(5));
        
        assert_eq!(*circle_5.get_radius(), Scalar::new(5));

        // assert_eq!(circle_5.get_area(), Scalar::new());
    }
}
