mod gf163;

use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, RemAssign, SubAssign};
use num_bigint::BigUint;
use num_traits::{Num, NumAssignOps, NumAssignRef, NumOps, NumRef, One, RefNum, Zero};

/// GFPB - Galois Field Polynomial Basis
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct GF239 {
  poly: BigUint,
  prime_poly: BigUint,
  polynomial_deg: u16,
}

// todo: create macros for creating Polynomial in BigUint and assign poly as constant
//  poly!([239, 15, 2, 1, 0]) and get poly in hex String to assign another one GF239 structs
//  or maybe create some static variable for holding BigUint value for module purpose
// todo: create macros for creating common arithmetic
//  gf_arithmetic!(239, poly!([239, 15, 2, 1, 0]))

fn create_prime_polynomial<T: AsRef<[u32]>>(degs: &T) -> BigUint {
  let mut prime_poly = BigUint::zero();
  for deg in degs.as_ref() {
    prime_poly += BigUint::one() << *deg;
  }
  prime_poly
}

impl GF239 {
  pub fn get_prime_poly(&self) -> BigUint {
    self.prime_poly.clone()
  }
}

impl One for GF239 {
  fn one() -> Self {
    GF239 {
      poly: BigUint::one(),
      prime_poly: create_prime_polynomial(&[239, 15, 2, 1, 0]),
      polynomial_deg: 239,
    }
  }
}

impl Zero for GF239 {
  fn zero() -> Self {
    GF239 {
      poly: BigUint::zero(),
      prime_poly: create_prime_polynomial(&[239, 15, 2, 1, 0]),
      polynomial_deg: 239,
    }
  }

  fn is_zero(&self) -> bool {
    self.poly.is_zero()
  }
}

impl Mul<Self> for GF239 {
  type Output = GF239;

  fn mul(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

impl Add<Self> for GF239 {
  type Output = GF239;

  fn add(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

