use std::{fmt, mem};
use std::fmt::Formatter;
use std::ops::{Add, Mul};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::gf::create_prime_polynomial;
use crate::gf::gf_def::{GFArithmetic, GFFactory};
use crate::gf_arithmetic::addition::add;
use crate::gf_arithmetic::multiplication::{inverse, mul, pow, square};
use crate::gf_arithmetic::reduction::module_reduction;
use crate::gf_arithmetic::trace::{htrace, trace};
use crate::helpers::{to_binary_be, to_binary_le, to_lower_hex_be, to_lower_hex_le, to_upper_hex_be, to_upper_hex_le};

/// GFPB - Galois Field Polynomial Basis
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct GF239 {
  pub poly: BigUint,
  prime_poly: BigUint,
}

impl GFFactory<'_> for GF239 {
  fn new(poly: BigUint, prime_poly: BigUint) -> Self {
    GF239 { poly, prime_poly }
  }

  fn create_prime_poly() -> BigUint {
    // prime_poly: create_prime_polynomial(&[239, 15, 2, 1, 0]),
    create_prime_polynomial(&[3, 1, 0])
  }
}
impl<'a> GFArithmetic<'a> for GF239 {
  fn get_prime_poly(&self) -> BigUint {
    self.prime_poly.clone()
  }

  fn get_value(&self) -> BigUint {
    self.poly.clone()
  }
}

impl From<BigUint> for GF239 {
  fn from(mut value: BigUint) -> Self {
    let mut num = GF239::zero();
    module_reduction(&mut value, &num.prime_poly);
    let _ = mem::replace(&mut num.poly, value);
    num
  }
}

impl One for GF239 {
  fn one() -> Self {
    GF239 {
      poly: BigUint::one(),
      prime_poly: Self::create_prime_poly(),
    }
  }
}

impl Zero for GF239 {
  fn zero() -> Self {
    GF239 {
      poly: BigUint::zero(),
      prime_poly: Self::create_prime_poly(),
    }
  }

  fn is_zero(&self) -> bool {
    self.poly.is_zero()
  }
}

impl Mul<Self> for GF239 {
  type Output = GF239;

  fn mul(mut self, rhs: Self) -> Self::Output {
    mul(&mut self.poly, &rhs.poly, &self.prime_poly);
    self
  }
}

impl Add<Self> for GF239 {
  type Output = GF239;

  fn add(mut self, rhs: Self) -> Self::Output {
    add(&mut self.poly, &rhs.poly);
    self
  }
}

impl Mul<&Self> for GF239 {
  type Output = GF239;

  fn mul(mut self, rhs: &Self) -> Self::Output {
    mul(&mut self.poly, &rhs.poly, &self.prime_poly);
    self
  }
}

impl Add<&Self> for GF239 {
  type Output = GF239;

  fn add(mut self, rhs: &Self) -> Self::Output {
    add(&mut self.poly, &rhs.poly);
    self
  }
}
impl Mul<Self> for &GF239 {
  type Output = GF239;

  fn mul(mut self, rhs: Self) -> Self::Output {
    let mut num = self.clone();
    mul(&mut num.poly, &rhs.poly, &num.prime_poly);
    num
  }
}

impl Add<Self> for &GF239 {
  type Output = GF239;

  fn add(mut self, rhs: Self) -> Self::Output {
    let mut num = self.clone();
    add(&mut num.poly, &rhs.poly);
    num
  }
}
impl Mul<&Self> for &GF239 {
  type Output = GF239;

  fn mul(mut self, rhs: &Self) -> Self::Output {
    let mut num = self.clone();
    mul(&mut num.poly, &rhs.poly, &num.prime_poly);
    num
  }
}

impl Add<&Self> for &GF239 {
  type Output = GF239;

  fn add(self, rhs: &Self) -> Self::Output {
    let mut num = self.clone();
    add(&mut num.poly, &rhs.poly);
    num
  }
}

impl fmt::Debug for GF239 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!("{:X?}", self.poly))
  }
}

impl fmt::Display for GF239 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!("{:X?}", self.poly))
  }
}

impl fmt::LowerHex for GF239 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", to_lower_hex_be(&self.poly))
  }
}

impl fmt::UpperHex for GF239 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", to_upper_hex_be(&self.poly))
  }
}

impl fmt::Binary for GF239 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", to_binary_be(&self.poly))
  }
}
