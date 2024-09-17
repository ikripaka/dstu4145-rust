pub fn get_string_hex_array_plain(arr: &[u8]) -> String {
  let mut res = format!("{:02X?}", arr);
  res = res.replace(", ", "");
  res = res.trim_start_matches('[').to_string();
  res = res.trim_end_matches(']').to_string();
  res = res.replace(' ', "");
  res
}
pub fn get_string_array_plain<T: AsRef<str>>(s: &T) -> String {
  let mut res = String::from(s.as_ref());
  res = res.replace(", ", "");
  res = res.trim_start_matches('[').to_string();
  res = res.trim_end_matches(']').to_string();
  res = res.replace(' ', "");
  res
}
