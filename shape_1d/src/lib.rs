pub mod scalars;
use std::ops::{Add, Sub, Mul, Div};

pub use scalars::scalar::Scalar;

pub trait GenericTypeAlias<T> : Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy {}

pub trait Shape1d<T>
{
    fn get_value(&self) -> &T;
    fn set_value(&mut self, val: T) -> &Self;
    fn get_mut_value(&mut self) -> &mut T;
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::scalars::line::Line;
    // use crate::scalars::scalar::Scalar;

    // #[test]
    // fn line_1() {
    //     let line = Line::new(Scalar::new(5));
    //     let value = *line.get_value();
    //     assert_eq!(value, 5);
    // }

    // fn line_2() {
    //     let mut line = Line::new(Scalar::new(5));

    //     let mut value = line.get_value();
    //     *value = 10;

    //     assert_eq!(*value, 10);
    // }
}
