use std::ops::{Add, Sub, Mul, Div};

use shape_1d::Scalar;
use crate::Shape2d;

pub struct Square<T = u32> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
  length : Scalar<T>
}

impl<T> Shape2d<T> for Square<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn get_area(&self) -> Scalar<T> {
        return *self.get_length() * *self.get_length();
    }
} 

impl<T> Square<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn new(length : Scalar<T>) -> Self {
        return Self {
            length: length,
        };
    }

    fn get_length(&self) -> &Scalar<T> {
        return &self.length;
    }
}

#[cfg(test)]
mod tests {
    use shape_1d::Scalar;
    use crate::{shapes::square::Square, Shape2d};

    #[test]
    fn test_square_properties() {
        let square_5 = Square::new(Scalar::new( 5));
        
        assert_eq!(*square_5.get_length(), Scalar::new(5));

        assert_eq!(square_5.get_area(), Scalar::new(25));
    }
}
