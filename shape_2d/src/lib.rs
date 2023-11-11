pub mod shapes;
use shape_1d::{scalars::scalar::Scalar, Shape1d};

pub trait Shape2d <T> {
    fn get_area(self) -> Scalar<T>;
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
