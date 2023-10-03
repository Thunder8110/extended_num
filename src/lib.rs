pub mod extended_num;

#[cfg(test)]
mod tests {
  use crate::extended_num::ExtendedNum::{PosInf, Finite, NegInf};

  #[test]
  fn test_finite() {
    let one = Finite(1);
    let two = Finite(2);
    let three = Finite(3);
    let six = Finite(6);
    assert!(one < two);
    assert!(three == Finite(3));
    assert_eq!(one + two, three);
    assert_eq!(three - two, one);
    assert_eq!(two * three, six);
    assert_eq!(six / three, two);
    assert_eq!(three / two, one);
  }

  #[test]
  fn test_infinite() {
    let posinf = PosInf;
    let neginf = NegInf;
    let four = Finite(4);
    let m_five = Finite(-5);
    assert!(neginf < four);
    assert!(four < posinf);
    assert_eq!(posinf + four, posinf);
    assert_eq!(posinf + posinf, posinf);
    assert_eq!(four + neginf, neginf);
    assert_eq!(neginf + neginf, neginf);
    assert_eq!(posinf - neginf, posinf);
    assert_eq!(four - posinf, neginf);
    assert_eq!(posinf * four, posinf);
    assert_eq!(four * neginf, neginf);
    assert_eq!(posinf * m_five, neginf);
    assert_eq!(m_five * neginf, posinf);
    assert_eq!(posinf / four, posinf);
    assert_eq!(m_five / neginf, Finite(0));
  }
}
