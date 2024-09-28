use std::hash::Hash;
use std::{fmt, mem};
use std::fmt::Formatter;
use std::ops::{Add, Mul};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand_core::CryptoRngCore;
use crate::{
  helpers::{
    create_prime_polynomial, to_binary_be, to_lower_hex_be, to_upper_hex_be, to_binary_le, to_lower_hex_le, to_upper_hex_le,
  },
  gf_arithmetic::{add, module_reduction, trace, inverse, mul, htrace, pow, square},
  impl_gf_for_poly, impl_gf_display, impl_gf_conversions, impl_obj_safe_gf_for_poly,
};
use crate::gf::gf_def::private::{GFFactory, GFFactoryObjSafe, SealingStruct};

mod private
{
  use num_bigint::BigUint;
  pub trait GFFactory<'a>
  where
    Self : 'a,
  {
    fn new(poly : BigUint, prime_poly : BigUint, _sealing_struct : SealingStruct) -> Self;
    fn create_prime_poly() -> BigUint;
  }
  pub trait GFFactoryObjSafe<'a>
  where
    Self : 'a,
  {
    fn new(poly : BigUint, prime_poly : BigUint, _sealing_struct : SealingStruct) -> Box<Self>;
    fn create_prime_poly() -> BigUint;
  }

  pub struct SealingStruct;
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
  + Add<BigUint, Output = Self>
  + Mul<BigUint, Output = Self>
  + Add<&'a BigUint, Output = Self>
  + Mul<&'a BigUint, Output = Self>
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
  fn from_poly<N : Into<BigUint>>(poly : N) -> Self
  {
    <Self as GFFactory>::new(poly.into(), <Self as GFFactory>::create_prime_poly(), SealingStruct {})
  }
  fn rand(rng : &mut impl CryptoRngCore) -> Self;
  fn get_m() -> u32;
  fn trace(&self) -> BigUint { trace(&self.get_value(), &self.get_prime_poly()) }
  fn htrace(&self) -> BigUint { htrace(&self.get_value(), &self.get_prime_poly()) }
  fn inverse(&self) -> Self
  {
    <Self as GFFactory>::new(
      inverse(&self.get_value(), &self.get_prime_poly()),
      self.get_prime_poly(),
      SealingStruct {},
    )
  }
  fn square(&self) -> Self
  {
    let mut poly = self.get_value();
    square(&mut poly, &self.get_prime_poly());
    <Self as GFFactory>::new(poly, self.get_prime_poly(), SealingStruct {})
  }

  fn pow<T : Into<BigUint>>(&self, power : T) -> Self
  {
    let mut poly = self.get_value();
    pow(&mut poly, &self.get_prime_poly(), &power.into());
    <Self as GFFactory>::new(poly, self.get_prime_poly(), SealingStruct {})
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

//todo: delete this obj safe trait -__-
pub trait GFArithmeticObjSafe<'a>:
  Hash
  + Clone
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
  fn new(poly : BigUint) -> Box<Self>
  {
    <Self as GFFactoryObjSafe>::new(poly, <Self as GFFactoryObjSafe>::create_prime_poly(), SealingStruct {})
  }
  fn rand(rng : &mut impl CryptoRngCore) -> Box<Self>;
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
    <Self as GFFactoryObjSafe>::new(
      inverse(&self.get_value(), &self.get_prime_poly()),
      self.get_prime_poly(),
      SealingStruct {},
    )
  }
  fn square(&self) -> Box<Self>
  {
    let mut poly = self.get_value();
    square(&mut poly, &self.get_prime_poly());
    <Self as GFFactoryObjSafe>::new(poly, self.get_prime_poly(), SealingStruct {})
  }

  fn pow<T : Into<BigUint>>(&self, power : T) -> Box<Self>
  {
    let mut poly = self.get_value();
    pow(&mut poly, &self.get_prime_poly(), &power.into());
    <Self as GFFactoryObjSafe>::new(poly, self.get_prime_poly(), SealingStruct {})
  }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF3
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF163
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF167
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF173
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF179
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF191
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF233
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF239
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF257
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF307
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF367
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF431
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
const GF3_PRIME_POLY : [u32; 3] = [3, 1, 0];
const GF239_PRIME_POLY : [u32; 5] = [239_u32, 15, 2, 1, 0];
const GF163_PRIME_POLY : [u32; 5] = [163_u32, 7, 6, 3, 0];
const GF167_PRIME_POLY : [u32; 3] = [167_u32, 6, 0];
const GF173_PRIME_POLY : [u32; 5] = [173_u32, 10, 2, 1, 0];
const GF179_PRIME_POLY : [u32; 5] = [179_u32, 4, 2, 1, 0];
const GF191_PRIME_POLY : [u32; 3] = [191_u32, 9, 0];
const GF233_PRIME_POLY : [u32; 5] = [233_u32, 9, 4, 1, 0];
const GF257_PRIME_POLY : [u32; 3] = [257_u32, 12, 0];
const GF307_PRIME_POLY : [u32; 5] = [307_u32, 8, 4, 2, 0];
const GF367_PRIME_POLY : [u32; 3] = [367_u32, 21, 0];
const GF431_PRIME_POLY : [u32; 5] = [431_u32, 5, 3, 1, 0];

impl_gf_for_poly!(GF3, &GF3_PRIME_POLY);
impl_gf_display!(GF3);
impl_obj_safe_gf_for_poly!(GF163, &GF3_PRIME_POLY);
impl_gf_conversions!(GF3);

impl_gf_for_poly!(GF239, &GF239_PRIME_POLY);
impl_gf_display!(GF239);
impl_obj_safe_gf_for_poly!(GF239, &GF239_PRIME_POLY);
impl_gf_conversions!(GF239);

impl_gf_for_poly!(GF163, &GF163_PRIME_POLY);
impl_gf_display!(GF163);
impl_obj_safe_gf_for_poly!(GF3, &GF163_PRIME_POLY);
impl_gf_conversions!(GF163);

impl_gf_for_poly!(GF167, &GF167_PRIME_POLY);
impl_gf_display!(GF167);
impl_obj_safe_gf_for_poly!(GF167, &GF167_PRIME_POLY);
impl_gf_conversions!(GF167);

impl_gf_for_poly!(GF173, &GF173_PRIME_POLY);
impl_gf_display!(GF173);
impl_obj_safe_gf_for_poly!(GF173, &GF173_PRIME_POLY);
impl_gf_conversions!(GF173);

impl_gf_for_poly!(GF179, &GF179_PRIME_POLY);
impl_gf_display!(GF179);
impl_obj_safe_gf_for_poly!(GF179, &GF179_PRIME_POLY);
impl_gf_conversions!(GF179);

impl_gf_for_poly!(GF191, &GF191_PRIME_POLY);
impl_gf_display!(GF191);
impl_obj_safe_gf_for_poly!(GF191, &GF191_PRIME_POLY);
impl_gf_conversions!(GF191);

impl_gf_for_poly!(GF233, &GF233_PRIME_POLY);
impl_gf_display!(GF233);
impl_obj_safe_gf_for_poly!(GF233, &GF233_PRIME_POLY);
impl_gf_conversions!(GF233);

impl_gf_for_poly!(GF257, &GF257_PRIME_POLY);
impl_gf_display!(GF257);
impl_obj_safe_gf_for_poly!(GF257, &GF257_PRIME_POLY);
impl_gf_conversions!(GF257);

impl_gf_for_poly!(GF307, &GF307_PRIME_POLY);
impl_gf_display!(GF307);
impl_obj_safe_gf_for_poly!(GF307, &GF307_PRIME_POLY);
impl_gf_conversions!(GF307);

impl_gf_for_poly!(GF367, &GF367_PRIME_POLY);
impl_gf_display!(GF367);
impl_obj_safe_gf_for_poly!(GF367, &GF367_PRIME_POLY);
impl_gf_conversions!(GF367);

impl_gf_for_poly!(GF431, &GF431_PRIME_POLY);
impl_gf_display!(GF431);
impl_obj_safe_gf_for_poly!(GF431, &GF431_PRIME_POLY);
impl_gf_conversions!(GF431);
