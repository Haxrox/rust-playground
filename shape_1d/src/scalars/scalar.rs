use std::ops::{Deref, DerefMut};

use crate::Shape1d;

#[derive(Debug)]
pub struct Scalar<T> {
  magnitude : T
}

impl<T> Scalar<T> {
  pub fn new(magnitude : T) -> Self {
    return Self {
      magnitude
    };
  }
}

impl<T> Shape1d<T> for Scalar<T> {
  fn get_value(&self) -> &T {
    return &self.magnitude;
  }

  fn get_mut_value(&mut self) -> &mut T {
    return &mut self.magnitude;
  }

  fn set_value(&mut self, val : T) -> &Self {
    self.magnitude = val;
    return self;
  }
}

impl<T> Deref for Scalar<T> { 
  type Target = T;
  
  fn deref(&self) -> &Self::Target {
      return self.get_value();
  }
}

impl<T> DerefMut for Scalar<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
      return self.get_mut_value();
  }
}

#[cfg(test)]
mod tests {
  use crate::Shape1d;
  use super::Scalar;

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
  fn scalar_get_mut_value_2() {
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
  fn scalar_get_mut_value_3() {
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
  fn scalar_get_mut_value_4() {
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
  fn scalar_get_mut_value_5() {
    let mut five: Scalar<i32> = Scalar::new(5);
    assert_eq!(*five, 5);

    *five = 10;
    assert_eq!(*five, 10);
  }
}