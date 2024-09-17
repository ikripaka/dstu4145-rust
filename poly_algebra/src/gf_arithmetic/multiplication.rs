use num_bigint::BigUint;
use num_traits::One;

pub fn mul(a: &mut BigUint, b: &BigUint, prime_poly: &BigUint, poly_size: u32) {}

pub fn square(a: &mut BigUint, prime_poly: &BigUint, poly_size: u32) {}

/// Reverse by multiplication
pub fn reverse(a: &BigUint, prime_poly: &BigUint, poly_size: u32) -> BigUint {
  BigUint::one()
}
