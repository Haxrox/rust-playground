use crate::Shape2d;
use shape_1d::{scalars::scalar::Scalar, Shape1d};

pub struct Rectangle<T = u32> {
    length : Scalar<T>,
    width : Scalar<T>
}

impl<T> Shape2d<T> for Rectangle<T> {
    fn get_area(self) -> Scalar<T> {
        return self.get_length() * self.get_width();
    }
} 


impl<T> Rectangle<T> {
    fn get_length(self) -> Scalar<T> {
        return self.length;
    }

    fn get_width(self) -> Scalar<T> {
        return self.width;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
