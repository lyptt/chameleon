use num_traits::PrimInt;

pub fn div_up<T: PrimInt>(a: T, b: T) -> T {
  let whole = a / b;
  let part = a % b;

  match part > T::from(0).unwrap() && whole >= T::from(1).unwrap() {
    true => whole + T::from(1).unwrap(),
    false => whole,
  }
}
