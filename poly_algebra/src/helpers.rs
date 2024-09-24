use num_bigint::BigUint;

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
pub fn to_binary_le(x: &BigUint) -> String {
  let mut tmp = x.to_radix_le(2);
  let tmp = tmp.iter().map(|x| format!("{:b}", x)).collect::<Vec<String>>();
  let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
    acc += &x;
    acc
  });
  tmp
}

pub fn to_binary_be(x: &BigUint) -> String {
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

pub fn to_lower_hex_le(x: &BigUint) -> String {
  let mut tmp = x.to_radix_le(16);
  let tmp = tmp.iter().map(|x| format!("{:x?}", x)).collect::<Vec<String>>();
  let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
    acc += &x;
    acc
  });
  tmp
}

pub fn to_lower_hex_be(x: &BigUint) -> String {
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

pub fn to_upper_hex_le(x: &BigUint) -> String {
  to_lower_hex_le(x).to_uppercase()
}
pub fn to_upper_hex_be(x: &BigUint) -> String {
  to_lower_hex_be(x).to_uppercase()
}
