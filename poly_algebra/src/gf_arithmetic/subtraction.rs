use num_bigint::BigUint;
use crate::gf_arithmetic::reduction::module_reduction;

pub fn sub(a: &mut BigUint, b: &BigUint, prime_poly: &BigUint, poly_size: u32) {
  *a ^= b;
  module_reduction(a, prime_poly, poly_size);
}
