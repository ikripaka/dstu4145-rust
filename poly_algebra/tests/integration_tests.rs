#[cfg(test)]
mod tests {
  use num_bigint::BigUint;
  use num_traits::{Num, Zero};
  use poly_algebra::gf::GF239;

  pub fn get_string_array_plain<T: AsRef<str>>(s: &T) -> String {
    let mut res = String::from(s.as_ref());
    res = res.replace(", ", "");
    res = res.trim_start_matches('[').to_string();
    res = res.trim_end_matches(']').to_string();
    res = res.replace(' ', "");
    res
  }

  pub fn get_string_hex_array_plain(arr: &[u8]) -> String {
    let mut res = format!("{:02X?}", arr);
    res = res.replace(", ", "");
    res = res.trim_start_matches('[').to_string();
    res = res.trim_end_matches(']').to_string();
    res = res.replace(' ', "");
    res
  }
  pub(crate) fn to_binary_le(x: &BigUint) -> String {
    let mut tmp = x.to_radix_le(2);
    let tmp = tmp.iter().map(|x| format!("{:b}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
      acc += &x;
      acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
      "0".to_string()
    } else {
      x.to_string()
    }
  }

  pub(crate) fn to_binary_be(x: &BigUint) -> String {
    let mut tmp = x.to_radix_be(2);
    let tmp = tmp.iter().map(|x| format!("{:b}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
      acc += &x;
      acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
      "0".to_string()
    } else {
      x.to_string()
    }
  }

  pub(crate) fn to_lower_hex_le(x: &BigUint) -> String {
    let mut tmp = x.to_radix_le(16);
    let tmp = tmp.iter().map(|x| format!("{:x?}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
      acc += &x;
      acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
      "0".to_string()
    } else {
      x.to_string()
    }
  }

  pub(crate) fn to_lower_hex_be(x: &BigUint) -> String {
    let mut tmp = x.to_radix_be(16);
    let tmp = tmp.iter().map(|x| format!("{:x?}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
      acc += &x;
      acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
      "0".to_string()
    } else {
      x.to_string()
    }
  }

  pub(crate) fn to_upper_hex_le(x: &BigUint) -> String {
    to_lower_hex_le(x).to_uppercase()
  }
  pub(crate) fn to_upper_hex_be(x: &BigUint) -> String {
    to_lower_hex_be(x).to_uppercase()
  }
  #[test]
  fn prime_poly_test() {
    // assert_eq!(GF163::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF167::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF173::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF179::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF191::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF233::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    assert_eq!(
      GF239::zero().get_prime_poly(),
      BigUint::from_str_radix("800000000000000000000000000000000000000000000000000000008007", 16).unwrap()
    );
    // assert_eq!(GF257::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF307::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF367::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF431::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // println!("{:?}",to_upper_hex_be(&GF239::zero().get_prime_poly()));
  }
}
