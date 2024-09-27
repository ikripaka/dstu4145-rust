use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Mul};
use num_bigint::BigUint;
use crate::gf_arithmetic::multiplication::{inverse, pow, square};
use crate::gf_arithmetic::trace::{htrace, trace};
use crate::helpers::{to_binary_be, to_binary_le, to_lower_hex_be, to_lower_hex_le, to_upper_hex_be, to_upper_hex_le};

pub(crate) use private::GFFactory;
pub(crate) use private::GFFactoryObjSafe;

mod private
{
  use num_bigint::BigUint;
  pub trait GFFactory<'a>
  where
    Self : 'a,
  {
    fn new(poly : BigUint, prime_poly : BigUint) -> Self;
    fn create_prime_poly() -> BigUint;
  }
  pub trait GFFactoryObjSafe<'a>
  where
    Self : 'a,
  {
    fn new(poly : BigUint, prime_poly : BigUint) -> Box<Self>;
    fn create_prime_poly() -> BigUint;
  }
}

pub trait GFArithmetic<'a>:
  Hash
  + Clone
  + PartialEq
  + Eq
  + Add<Self, Output = Self>
  + Mul<Self, Output = Self>
  + Add<&'a Self, Output = Self>
  + Mul<&'a Self, Output = Self>
  + fmt::Debug
  + fmt::Display
  + fmt::LowerHex
  + fmt::UpperHex
  + fmt::Binary
  + num_traits::One
  + num_traits::Zero
  + From<BigUint>
  + Into<BigUint>
  + GFFactory<'a>
  + GFGetters
  + GFDisplay
{
  fn new(poly : BigUint) -> Self { <Self as GFFactory>::new(poly, <Self as GFFactory>::create_prime_poly()) }
  fn trace(&self) -> BigUint { trace(&self.get_value(), &self.get_prime_poly()) }
  fn htrace(&self) -> BigUint { htrace(&self.get_value(), &self.get_prime_poly()) }
  fn inverse(&self) -> Self
  {
    <Self as GFFactory>::new(inverse(&self.get_value(), &self.get_prime_poly()), self.get_prime_poly())
  }
  fn square(&self) -> Self
  {
    let mut poly = self.get_value();
    square(&mut poly, &self.get_prime_poly());
    <Self as GFFactory>::new(poly, self.get_prime_poly())
  }

  fn pow<T : Into<BigUint>>(&self, power : T) -> Self
  {
    let mut poly = self.get_value();
    pow(&mut poly, &self.get_prime_poly(), &power.into());
    <Self as GFFactory>::new(poly, self.get_prime_poly())
  }
}

pub trait GFGetters
{
  fn get_prime_poly(&self) -> BigUint;
  fn get_value(&self) -> BigUint;
}
pub trait GFDisplay: GFGetters
{
  fn to_binary_le(&self) -> String { to_binary_le(&self.get_value()) }
  fn to_binary_be(&self) -> String { to_binary_be(&self.get_value()) }
  fn to_lower_hex_le(&self) -> String { to_lower_hex_le(&self.get_value()) }
  fn to_lower_hex_be(&self) -> String { to_lower_hex_be(&self.get_value()) }
  fn to_upper_hex_le(&self) -> String { to_upper_hex_le(&self.get_value()) }
  fn to_upper_hex_be(&self) -> String { to_upper_hex_be(&self.get_value()) }
}

pub trait GFArithmeticObjSafe<'a>:
  Hash
  + Clone
  + PartialEq
  + Eq
  + GFFactoryObjSafe<'a>
  + fmt::Debug
  + fmt::Display
  + fmt::LowerHex
  + fmt::UpperHex
  + fmt::Binary
  + From<BigUint>
  + Into<BigUint>
  + GFGetters
  + GFDisplay
{
  fn new(poly : BigUint) -> Box<Self> { <Self as GFFactoryObjSafe>::new(poly, <Self as GFFactoryObjSafe>::create_prime_poly()) }
  fn one() -> Box<Self>;
  fn is_one(&self) -> bool;
  fn zero() -> Box<Self>;
  fn is_zero(&self) -> bool;
  fn add(self, other : &Self) -> Box<Self>;
  fn mul(self, other : &Self) -> Box<Self>;
  fn trace(&self) -> BigUint { trace(&self.get_value(), &self.get_prime_poly()) }
  fn htrace(&self) -> BigUint { htrace(&self.get_value(), &self.get_prime_poly()) }
  fn inverse(&self) -> Box<Self>
  {
    <Self as GFFactoryObjSafe>::new(inverse(&self.get_value(), &self.get_prime_poly()), self.get_prime_poly())
  }
  fn square(&self) -> Box<Self>
  {
    let mut poly = self.get_value();
    square(&mut poly, &self.get_prime_poly());
    <Self as GFFactoryObjSafe>::new(poly, self.get_prime_poly())
  }

  fn pow<T : Into<BigUint>>(&self, power : T) -> Box<Self>
  {
    let mut poly = self.get_value();
    pow(&mut poly, &self.get_prime_poly(), &power.into());
    <Self as GFFactoryObjSafe>::new(poly, self.get_prime_poly())
  }
}
