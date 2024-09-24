use num_bigint::BigUint;
use crate::gf_arithmetic::addition::add;
use crate::helpers::to_binary_be;

pub fn module_reduction(a: &mut BigUint, prime_poly: &BigUint) {
  let mut a_bits = a.bits();
  let poly_size = prime_poly.bits();
  while a_bits >= poly_size {
    add(a, &(prime_poly << (a_bits - poly_size)));
    a_bits = a.bits();
  }
}
