use std::hash::Hash;
use std::{fmt, mem};
use std::fmt::Formatter;
use std::ops::{Add, Mul};
use std::sync::LazyLock;
use num_bigint::{BigUint, ParseBigIntError};
use num_traits::{Num, One, Zero};
use rand_core::CryptoRngCore;
use crate::{
  helpers::{
    create_prime_polynomial, to_binary_be, to_lower_hex_be, to_upper_hex_be, to_binary_le, to_lower_hex_le, to_upper_hex_le,
  },
  gf_arithmetic::{add, module_reduction, trace, inverse, mul, htrace, pow, square},
  gf::gf_def::private::{GFFactory, SealingStruct},
  impl_gf_for_poly, impl_gf_display, impl_gf_conversions,
};

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
  pub struct SealingStruct;
}

/// Trait for reducing boilerplate code & implementing Galois field arithmetic.
/// Needles to note that, this trait isn't object safe, so you can't use it with `dyn` keyword.
///```compile_fail
/// # use num_bigint::BigUint;
/// # use num_traits::One;
/// # use poly_algebra::gf::GF163;
/// let mut x : Box<dyn poly_algebra::gf::GFArithmetic> = Box::new(GF163::one());
/// assert_eq!(x.get_value(), BigUint::one())
/// ```
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
  fn from_hex_be<N : AsRef<str>>(n : N) -> Result<Self, ParseBigIntError>
  {
    Ok(<Self as GFFactory>::new(
      BigUint::from_str_radix(n.as_ref(), 16)?,
      <Self as GFFactory>::create_prime_poly(),
      SealingStruct {},
    ))
  }
  fn from_binary_be<N : AsRef<str>>(n : N) -> Result<Self, ParseBigIntError>
  {
    Ok(<Self as GFFactory>::new(
      BigUint::from_str_radix(n.as_ref(), 2)?,
      <Self as GFFactory>::create_prime_poly(),
      SealingStruct {},
    ))
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

const GF163_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF163_PRIME_POLY));
const GF167_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF167_PRIME_POLY));
const GF173_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF173_PRIME_POLY));
const GF179_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF179_PRIME_POLY));
const GF191_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF191_PRIME_POLY));
const GF233_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF233_PRIME_POLY));
const GF257_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF257_PRIME_POLY));
const GF307_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF307_PRIME_POLY));
const GF367_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF367_PRIME_POLY));
const GF431_PRECALC_PRIME_POLY : LazyLock<BigUint> = LazyLock::new(|| create_prime_polynomial(&GF431_PRIME_POLY));
/// GF 2^163 over prime polynomial `x^163 + x^7 + x^6 + x^3 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF163
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^167 over prime polynomial `x^167 + x^6 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF167
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^173 over prime polynomial `x^173 + x^10 + x^2 + x + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF173
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^179 over prime polynomial `x^179 + x^4 + x^2 + x + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF179
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^191 over prime polynomial `x^191 + x^9 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF191
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^233 over prime polynomial `x^233 + x^9 + x^4 + x^1 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF233
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^257 over prime polynomial `x^257 + x^12 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF257
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^307 over prime polynomial `x^307 + x^8 + x^4 + x^2 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF307
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^367 over prime polynomial `x^367 + x^21 + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF367
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
/// GF 2^431 over prime polynomial `x^431 + x^5 + x^3 + x + 1`.
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF431
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
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

impl_gf_for_poly!(GF163, &GF163_PRIME_POLY, GF163_PRECALC_PRIME_POLY);
impl_gf_display!(GF163);
impl_gf_conversions!(GF163);

impl_gf_for_poly!(GF167, &GF167_PRIME_POLY, GF167_PRECALC_PRIME_POLY);
impl_gf_display!(GF167);
impl_gf_conversions!(GF167);

impl_gf_for_poly!(GF173, &GF173_PRIME_POLY, GF173_PRECALC_PRIME_POLY);
impl_gf_display!(GF173);
impl_gf_conversions!(GF173);

impl_gf_for_poly!(GF179, &GF179_PRIME_POLY, GF179_PRECALC_PRIME_POLY);
impl_gf_display!(GF179);
impl_gf_conversions!(GF179);

impl_gf_for_poly!(GF191, &GF191_PRIME_POLY, GF191_PRECALC_PRIME_POLY);
impl_gf_display!(GF191);
impl_gf_conversions!(GF191);

impl_gf_for_poly!(GF233, &GF233_PRIME_POLY, GF233_PRECALC_PRIME_POLY);
impl_gf_display!(GF233);
impl_gf_conversions!(GF233);

impl_gf_for_poly!(GF257, &GF257_PRIME_POLY, GF257_PRECALC_PRIME_POLY);
impl_gf_display!(GF257);
impl_gf_conversions!(GF257);

impl_gf_for_poly!(GF307, &GF307_PRIME_POLY, GF307_PRECALC_PRIME_POLY);
impl_gf_display!(GF307);
impl_gf_conversions!(GF307);

impl_gf_for_poly!(GF367, &GF367_PRIME_POLY, GF367_PRECALC_PRIME_POLY);
impl_gf_display!(GF367);
impl_gf_conversions!(GF367);

impl_gf_for_poly!(GF431, &GF431_PRIME_POLY, GF431_PRECALC_PRIME_POLY);
impl_gf_display!(GF431);
impl_gf_conversions!(GF431);
