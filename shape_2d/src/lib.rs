pub mod shapes;
use std::ops::{Sub, Add, Mul, Div};

use shape_1d::Scalar;

pub trait Shape2d <T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn get_area(&self) -> Scalar<T>;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
