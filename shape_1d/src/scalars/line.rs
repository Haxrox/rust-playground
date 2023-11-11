use crate::Shape1d;
use crate::scalar::Scalar;

pub struct Line<T> {
  value : Scalar<T>
}

impl<T> Line<T> {
  pub fn new(value : Scalar<T>) -> Self {
    return Line {
      value: value
    };
  }
}

impl<T> Shape1d<Scalar<T>> for Line<T> {
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