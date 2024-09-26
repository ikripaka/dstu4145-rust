use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Mul};
use num_bigint::BigUint;
use crate::gf_arithmetic::multiplication::{inverse, pow, square};
use crate::gf_arithmetic::trace::{htrace, trace};
use crate::helpers::{to_binary_be, to_binary_le, to_lower_hex_be, to_lower_hex_le, to_upper_hex_be, to_upper_hex_le};

pub(crate) use private::GFFactory;

mod private {
  use num_bigint::BigUint;
  pub trait GFFactory<'a>
  where
    Self: 'a,
  {
    fn new(poly: BigUint, prime_poly: BigUint) -> Self;
    fn create_prime_poly() -> BigUint;
  }
}

pub trait GFArithmetic<'a>:
  Hash
  + Clone
  + PartialEq
  + Eq
  + Add<Self>
  + Mul<Self>
  + Add<&'a Self>
  + Mul<&'a Self>
  + fmt::Debug
  + fmt::Display
  + fmt::LowerHex
  + fmt::UpperHex
  + fmt::Binary
  + num_traits::One
  + num_traits::Zero
  + From<BigUint>
  + GFFactory<'a>
{
  fn new(poly: BigUint) -> Self {
    <Self as GFFactory>::new(poly, <Self as GFFactory>::create_prime_poly())
  }
  fn get_prime_poly(&self) -> BigUint;
  fn get_value(&self) -> BigUint;
  fn trace(&self) -> BigUint {
    trace(&self.get_value(), &self.get_prime_poly())
  }
  fn htrace(&self) -> BigUint {
    htrace(&self.get_value(), &self.get_prime_poly())
  }
  fn inverse(&self) -> Self {
    <Self as GFFactory>::new(self.get_prime_poly(), inverse(&self.get_value(), &self.get_prime_poly()))
  }
  fn square(&mut self) -> Self {
    let mut poly = self.get_value();
    square(&mut poly, &self.get_prime_poly());
    <Self as GFFactory>::new(poly, self.get_prime_poly())
  }

  fn pow<T: Into<BigUint>>(&mut self, power: T) -> Self {
    let mut poly = self.get_value();
    pow(&mut poly, &self.get_prime_poly(), &power.into());
    <Self as GFFactory>::new(poly, self.get_prime_poly())
  }

  fn to_binary_le(&self) -> String {
    to_binary_le(&self.get_value())
  }
  fn to_binary_be(&self) -> String {
    to_binary_be(&self.get_value())
  }
  fn to_lower_hex_le(&self) -> String {
    to_lower_hex_le(&self.get_value())
  }
  fn to_lower_hex_be(&self) -> String {
    to_lower_hex_be(&self.get_value())
  }
  fn to_upper_hex_le(&self) -> String {
    to_upper_hex_le(&self.get_value())
  }
  fn to_upper_hex_be(&self) -> String {
    to_upper_hex_be(&self.get_value())
  }
}
