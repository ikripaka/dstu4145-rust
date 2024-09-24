use std::mem;
use num_bigint::BigUint;
use crate::gf_arithmetic::addition::add;
use crate::gf_arithmetic::multiplication::square;

pub fn trace(a: &BigUint, prime_poly: &BigUint) -> BigUint {
  let poly_size = prime_poly.bits();
  let mut t = a.clone();
  for _ in 1..poly_size - 1 {
    square(&mut t, prime_poly);
    add(&mut t, a);
  }
  t
}
pub fn htrace(a: &BigUint, prime_poly: &BigUint) -> BigUint {
  let poly_size = prime_poly.bits();
  // Half trace only accepts odd values
  assert_eq!(poly_size % 2, 0);
  let mut t = a.clone();
  for _ in 1..=((poly_size - 1) >> 1) {
    square(&mut t, prime_poly);
    square(&mut t, prime_poly);
    add(&mut t, a);
  }
  t
}
