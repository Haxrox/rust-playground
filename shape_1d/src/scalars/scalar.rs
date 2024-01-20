use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use crate::Shape1d;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    magnitude: T,
}

impl<T> Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    pub fn new(magnitude: T) -> Self {
        return Self { magnitude };
    }

    pub fn mult(&self) -> T {
        return *self.get_value() * *self.get_value();
    }
}

impl<T> Shape1d<T> for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn get_value(&self) -> &T {
        return &self.magnitude;
    }

    fn get_mut_value(&mut self) -> &mut T {
        return &mut self.magnitude;
    }

    fn set_value(&mut self, val: T) -> &Self {
        self.magnitude = val;
        return self;
    }
}

impl<T> Add for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Output = Scalar<T>;
    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(*self + *rhs);
    }
}

impl<T> Sub for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Output = Scalar<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(*self - *rhs);
    }
}

impl<T> Mul for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Output = Scalar<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        return Self::new(*self * *rhs);
    }
}

impl<T> Div for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Output = Scalar<T>;
    fn div(self, rhs: Self) -> Self::Output {
        return Self::new(*self / *rhs);
    }
}

impl<T> Deref for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return self.get_value();
    }
}

impl<T> DerefMut for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.get_mut_value();
    }
}

// impl From<i32> for Scalar<i32>
// {
//     fn from(value: i32) -> Self {
//         return Self {
//             magnitude : value
//         };
//     }
// }

impl Into<i32> for Scalar<i32> 
{
    fn into(self) -> i32 {
        return self.magnitude;
    }
}

// impl From<i64> for Scalar<i64>
// {
//     fn from(value: i64) -> Self {
//         return Self {
//             magnitude : value
//         };
//     }
// }

impl Into<i64> for Scalar<i64> 
{
    fn into(self) -> i64 {
        return self.magnitude;
    }
}

// impl From<f32> for Scalar<f32>
// {
//     fn from(value: f32) -> Self {
//         return Self {
//             magnitude : value
//         };
//     }
// }

impl Into<f32> for Scalar<f32> 
{
    fn into(self) -> f32 {
        return self.magnitude as f32;
    }
}

// impl From<f64> for Scalar<f64>
// {
//     fn from(value: f64) -> Self {
//         return Self {
//             magnitude : value
//         };
//     }
// }

impl Into<f64> for Scalar<f64> 
{
    fn into(self) -> f64 {
        return self.magnitude as f64;
    }
}

impl<T> From<T> for Scalar<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn from(value: T) -> Self {
        return Self {
            magnitude : value
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{Scalar, Shape1d};

    /*
     * Very basic get_value and compare
     */
    #[test]
    fn scalar_get_value1() {
        let five: Scalar<i32> = Scalar::new(5);
        let five_value: &i32 = five.get_value();

        assert_eq!(*five_value, 5);
    }

    /*
     * get_mut_value to test mutability of variables
     */
    #[test]
    fn scalar_get_mut_value_1() {
        let mut five: Scalar<i32> = Scalar::new(5);
        let five_value: &mut i32 = five.get_mut_value();

        *five_value = 10;
        assert_eq!(*five_value, 10);
        assert_eq!(*five, 10);
    }

    /*
     * Mutate get_mut_value variable
     */
    #[test]
    fn scalar_get_mut_value_2() {
        let mut five: Scalar<i32> = Scalar::new(5);
        let mut five_value: &mut i32 = five.get_mut_value();

        let mut binding = 10;
        five_value = &mut binding;

        assert_eq!(*five, 5);
        assert_eq!(*five_value, 10);
    }

    /*
     * Test mutability of get_mut_value
     */
    #[test]
    fn scalar_get_mut_value_3() {
        let mut five: Scalar<i32> = Scalar::new(5);
        let five_value: &mut i32 = five.get_mut_value();

        *five_value = 10;
        assert_eq!(*five_value, 10);
        assert_eq!(*five, 10);

        let five_value_mut: &mut i32 = five.get_mut_value();
        assert_eq!(*five_value_mut, 10);
        /*
        Adding this will create two mutable references to `five` (`five_value` and `five_value_mut`)
        assert_eq!(*five_value, 10);
        */

        *five.get_mut_value() = 15;
        assert_eq!(*five, 15);
    }

    /*
     * Test deref of Scalar
     */
    #[test]
    fn scalar_deref_1() {
        let mut five: Scalar<i32> = Scalar::new(5);
        assert_eq!(*five, 5);

        *five = 10;
        assert_eq!(*five, 10);
    }

    /*
     * Test set_value
     */
    #[test]
    fn scalar_set_value_1() {
        let mut five: Scalar<i32> = Scalar::new(5);
        assert_eq!(*five.get_value(), 5);

        five.set_value(10);
        assert_eq!(*five, 10);
    }

    /*
     * Test derived operation
     */
    #[test]
    fn scalar_operation_1() {
        let five = Scalar::new(5);
        let ten = Scalar::new(10);

        assert_eq!(five + ten, Scalar::new(15));
    }

    /*
     * Test From implementation
     */
    #[test]
    fn scalar_from() {
        let five: Scalar<i32> = Scalar::new(5);
        
        assert_eq!(five, 5.into());
        /*
         * Fails
         * assert_eq!(5.into(), five);
         */
        assert_eq!(Scalar::from(5), five);
        assert_eq!(five, Scalar::from(5));
    }

    /*
     * Test Into implementation
     */
    #[test]
    fn scalar_into() {
        let five: Scalar<i32> = Scalar::new(5);
        let five_i64 : Scalar<i64> = Scalar::new(5 as i64);
        
        assert_eq!(5, five.into());
        // <Scalar<i32> as Into<T>>::into(`, `)`
        assert_eq!(<Scalar<i32> as Into<i32>>::into(five), 5 as i32);
        assert_eq!(<Scalar<i64> as Into<i64>>::into(five_i64), 5 as i64);
        /*
         * Fails
         * assert_eq!(five.into(), 5);
         */
        // assert_eq!(five.into(), 5 as i32);

    }

    /*
     * Test T = f64
     */
    #[test]
    fn scalar_f64() {
        let five: Scalar<f64> = Scalar::new(5.0);
        let ten: Scalar<f64> = Scalar::new(10.0);

        assert_eq!(five + ten, Scalar::new(15.0));
    }

    /*
     * Test PI
     */
    #[test]
    fn scalar_pi() {
        let pi: Scalar<f64> = Scalar::new(std::f64::consts::PI);
        let ten: Scalar<i32> = Scalar::new(10);

        assert_eq!(pi, Scalar::new(std::f64::consts::PI));
        assert_eq!(pi * Scalar::from(ten), Scalar::new(std::f64::consts::PI));
    }
}
