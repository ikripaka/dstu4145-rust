use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand_core::CryptoRngCore;
use crate::gf::gf_def::GFArithmetic;

pub fn get_string_array_plain<T : AsRef<str>>(s : &T) -> String
{
  let mut res = String::from(s.as_ref());
  res = res.replace(", ", "");
  res = res.trim_start_matches('[').to_string();
  res = res.trim_end_matches(']').to_string();
  res = res.replace(' ', "");
  res
}

pub fn get_string_hex_array_plain(arr : &[u8]) -> String
{
  let mut res = format!("{:02X?}", arr);
  res = res.replace(", ", "");
  res = res.trim_start_matches('[').to_string();
  res = res.trim_end_matches(']').to_string();
  res = res.replace(' ', "");
  res
}
pub fn to_binary_le(x : &BigUint) -> String
{
  let mut tmp = x.to_radix_le(2);
  let tmp = tmp.iter().map(|x| format!("{:b}", x)).collect::<Vec<String>>();
  let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
    acc += &x;
    acc
  });
  tmp
}

pub fn to_binary_be(x : &BigUint) -> String
{
  let mut tmp = x.to_radix_be(2);
  let tmp = tmp.iter().map(|x| format!("{:b}", x)).collect::<Vec<String>>();
  let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
    acc += &x;
    acc
  });
  let x = tmp.trim_start_matches("0");
  if x.is_empty()
  {
    "0".to_string()
  }
  else
  {
    x.to_string()
  }
}

pub fn to_lower_hex_le(x : &BigUint) -> String
{
  let mut tmp = x.to_radix_le(16);
  let tmp = tmp.iter().map(|x| format!("{:x?}", x)).collect::<Vec<String>>();
  let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
    acc += &x;
    acc
  });
  tmp
}

pub fn to_lower_hex_be(x : &BigUint) -> String
{
  let mut tmp = x.to_radix_be(16);
  let tmp = tmp.iter().map(|x| format!("{:x?}", x)).collect::<Vec<String>>();
  let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
    acc += &x;
    acc
  });
  let x = tmp.trim_start_matches("0");
  if x.is_empty()
  {
    "0".to_string()
  }
  else
  {
    x.to_string()
  }
}

pub fn to_upper_hex_le(x : &BigUint) -> String { to_lower_hex_le(x).to_uppercase() }
pub fn to_upper_hex_be(x : &BigUint) -> String { to_lower_hex_be(x).to_uppercase() }

pub(crate) fn create_prime_polynomial<T : AsRef<[u32]>>(degs : &T) -> BigUint
{
  let mut prime_poly = BigUint::zero();
  for deg in degs.as_ref()
  {
    prime_poly ^= BigUint::one() << *deg;
  }
  prime_poly
}

const BITS_IN_U32 : u128 = 32;
const BYTES_INTO_U32 : u128 = 4;
const BITS_INTO_BYTE : u128 = 8;
pub fn generate_num<T : Into<u128>>(rng : &mut impl CryptoRngCore, bit_len : T) -> BigUint
{
  let bit_len = bit_len.into();
  let gen_tries = (&bit_len / BITS_INTO_BYTE) / BYTES_INTO_U32;
  let bits_to_fit = bit_len - gen_tries * BITS_IN_U32;
  let mut res = BigUint::zero();
  for i in 0 .. gen_tries
  {
    res = (res << BITS_IN_U32) | BigUint::from(rng.next_u32());
  }
  let mask = (1 << bits_to_fit) - 1;
  let finalizing_item = rng.next_u32() & mask;
  res = (res << bits_to_fit) | BigUint::from(finalizing_item);
  res
}

/// z^2 + uz = w; u,w \in GF(2^m)
pub fn solve_quadratic_equation_in_field<'a, T : GFArithmetic<'a>>(u : &T, w : &T) -> Option<(T, u8)>
{
  if u.is_zero()
  {
    // Because prime poly has one bit more that ordinary number
    let power = T::get_m() - 1;
    let power = BigUint::one() << power;
    return Some((w.pow(power), 1));
  }
  if w.is_zero()
  {
    return Some((T::zero(), 2));
  }
  let v = {
    let u_inv = u.inverse();
    let u_inv_squared = u_inv.square();
    w.clone() * u_inv_squared
  };
  let tr_v = v.trace();
  if tr_v.is_one()
  {
    return None;
  }
  let t = v.htrace();
  let z = u.clone() * t;
  Some((z, 2))
}

/// Function considers that hash comes into the function unchanged
/// (i.e. we don't have to specify L_H [hash length])
pub fn create_field_el_from_hash<'a, T : GFArithmetic<'a>, H : AsRef<[u8]>>(hash : H) -> T
{
  let hash = BigUint::from_bytes_be(hash.as_ref());
  let k = std::cmp::min(hash.bits(), T::get_m() as u64);
  let mask = (BigUint::one() << k) - BigUint::one();
  let h = hash & mask;
  if h.is_zero()
  {
    T::one()
  }
  else
  {
    T::from_poly(h)
  }
}
