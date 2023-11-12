use std::ops::{Add, Div, Mul, Sub};

use crate::{Scalar, Shape1d};

#[derive(Copy, Clone)]
pub struct Line<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    value: Scalar<T>
}

impl<T> Line<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    pub fn new(value: Scalar<T>) -> Self {
        return Line { value: value };
    }
}

impl<T> Shape1d<Scalar<T>> for Line<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn get_value(&self) -> &Scalar<T> {
        return &self.value;
    }

    fn set_value(&mut self, val: Scalar<T>) -> &Self {
        self.value = val;
        return self;
    }

    fn get_mut_value(&mut self) -> &mut Scalar<T> {
        return &mut self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Scalar, Shape1d};

    /*
     * Very basic get_value and compare
     */
    #[test]
    fn line_get_value() {
        let line_five = Line::new(Scalar::new(5));

        assert_eq!(**line_five.get_value(), 5);
        assert_eq!(*line_five.get_value(), Scalar::new(5));
    }
}
