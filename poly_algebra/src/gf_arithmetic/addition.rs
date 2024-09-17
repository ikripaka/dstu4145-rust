use num_bigint::BigUint;
use num_traits::One;
use crate::gf_arithmetic::reduction::module_reduction;

pub fn add(a: &mut BigUint, b: &BigUint, prime_poly: &BigUint, poly_size: u32) {
  *a += b;
  module_reduction(a, prime_poly, poly_size);
}

/// Reverse by addition
pub fn reverse(a: &BigUint, prime_poly: &BigUint, poly_size: u32) -> BigUint {
  BigUint::one()
}
