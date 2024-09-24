pub mod gf163;
pub mod gf239;
pub mod gf_def;
pub mod gf_expand;

use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, RemAssign, SubAssign};
use num_bigint::BigUint;
use num_traits::{Num, NumAssignOps, NumAssignRef, NumOps, NumRef, One, RefNum, Zero};

// todo: create macros for creating Polynomial in BigUint and assign poly as constant
//  poly!([239, 15, 2, 1, 0]) and get poly in hex String to assign another one GF239 structs
//  or maybe create some static variable for holding BigUint value for module purpose
// todo: create macros for creating common arithmetic
//  gf_arithmetic!(239, poly!([239, 15, 2, 1, 0]))

fn create_prime_polynomial<T: AsRef<[u32]>>(degs: &T) -> BigUint {
  let mut prime_poly = BigUint::zero();
  for deg in degs.as_ref() {
    prime_poly ^= BigUint::one() << *deg;
  }
  prime_poly
}
