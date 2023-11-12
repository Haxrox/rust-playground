use std::ops::{Add, Sub, Mul, Div};

use shape_1d::Scalar;
use crate::Shape2d;

pub struct Rectangle<T = u32> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    length : Scalar<T>,
    width : Scalar<T>
}

impl<T> Shape2d<T> for Rectangle<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn get_area(&self) -> Scalar<T> {
        return *self.get_length() * *self.get_width();
    }
} 


impl<T> Rectangle<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn new(length : Scalar<T>, width : Scalar<T>) -> Self {
        return Self {
            length: length,
            width: width
        };
    }

    fn get_length(&self) -> &Scalar<T> {
        return &self.length;
    }

    fn get_width(&self) -> &Scalar<T> {
        return &self.width;
    }
}

#[cfg(test)]
mod tests {
    use shape_1d::Scalar;
    use crate::{shapes::rectangle::Rectangle, Shape2d};

    #[test]
    fn test_rectangle_properties() {
        let rec_4x5 = Rectangle::new(Scalar::new(4), Scalar::new( 5));
        
        assert_eq!(*rec_4x5.get_length(), Scalar::new(4));
        assert_eq!(*rec_4x5.get_width(), Scalar::new(5));

        assert_eq!(rec_4x5.get_area(), Scalar::new(20));
    }
}
