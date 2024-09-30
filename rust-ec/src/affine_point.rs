use std::fmt;
use std::fmt::Formatter;
use num_bigint::BigUint;
use num_traits::Zero;
use rand_core::CryptoRngCore;
use poly_algebra::gf::{GFArithmetic};
use crate::binary_ec::BinaryEC;
use crate::helpers::{generate_random_affine_point, pack_affine_point, unpack_affine_point};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum AffinePoint<T>
{
  Point
  {
    x : T,
    y : T,
  },
  Infinity,
}

impl<'a, T : GFArithmetic<'a>> AffinePoint<T>
{
  pub fn rand(rng : &mut impl CryptoRngCore, ec : &BinaryEC<T>) -> Self { generate_random_affine_point(rng, ec) }

  pub fn is_inf(&self) -> bool
  {
    if let Self::Infinity = self
    {
      true
    }
    else
    {
      false
    }
  }

  pub fn neutral() -> AffinePoint<T> { AffinePoint::Infinity }

  pub fn negative(&self) -> AffinePoint<T>
  {
    match self
    {
      AffinePoint::Point { x, y } => AffinePoint::Point {
        x : x.clone(),
        y : x.clone() + y.clone(),
      },
      AffinePoint::Infinity => AffinePoint::Infinity,
    }
  }

  pub fn add(&self, ec : &BinaryEC<T>, q : &Self) -> Self
  {
    match self
    {
      AffinePoint::Point { x: x_p, y: y_p } =>
      {
        let (x_p, y_p) = (x_p.clone(), y_p.clone());
        match q
        {
          AffinePoint::Point { x: x_q, y: y_q } =>
          {
            let (x_q, y_q) = (x_q.clone(), y_q.clone());
            if *self == q.negative()
            {
              return AffinePoint::Infinity;
            }
            if self == q
            {
              return self.double(ec);
            }
            /// Calculating `x_r`
            let numerator = y_p.clone() + y_q;
            let denominator = (x_p.clone() + x_q.clone()).inverse();
            let fraction = numerator.clone() * denominator.clone();
            let x_r = {
              let fraction2 = fraction.clone().square();
              fraction.clone() + fraction2.clone() + x_p.clone() + x_q.clone() + ec.a.as_field_el()
            };
            /// Calculating `y_r`
            let y_r = {
              let tmp1 = x_p.clone() + x_r.clone();
              fraction * tmp1 + x_r.clone() + y_p
            };
            AffinePoint::Point { x : x_r, y : y_r }
          }
          AffinePoint::Infinity => AffinePoint::Infinity,
        }
      }
      AffinePoint::Infinity => q.clone(),
    }
  }

  pub fn double(&self, ec : &BinaryEC<T>) -> Self
  {
    match self
    {
      AffinePoint::Point { x: x_p, y: y_p } =>
      {
        let x_p_squared = x_p.clone().square();
        let x_r = {
          let x_p_squared_inv = x_p_squared.inverse();
          x_p_squared.clone() + (ec.b.clone() * x_p_squared_inv)
        };
        let y_r = {
          let x_p_inv = x_p.inverse();
          x_p_squared + (x_p.clone() + (y_p.clone() * x_p_inv)) * x_r.clone() + x_r.clone()
        };
        AffinePoint::Point { x : x_r, y : y_r }
      }
      AffinePoint::Infinity => AffinePoint::Infinity,
    }
  }

  pub fn mul<N : Into<BigUint>>(&self, ec : &BinaryEC<T>, n : N) -> Self
  {
    let mut r = AffinePoint::neutral();
    let mut tmp = self.clone();
    let n = n.into();
    println!("n in mul: {}", n.to_str_radix(16));
    // from LSB to MSB
    for x in n.to_str_radix(2).into_bytes().iter().rev()
    {
      if *x - b'0' == 1
      {
        r = r.add(ec, &tmp)
      }
      tmp = tmp.add(ec, &tmp)
    }
    r
  }

  pub fn unpack(num : &T, ec : &BinaryEC<T>) -> Self { unpack_affine_point(num, ec) }

  pub fn pack(&self) -> T { pack_affine_point(self) }
}
impl<'a, T : GFArithmetic<'a>> fmt::Debug for AffinePoint<T>
{
  fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result
  {
    match self
    {
      AffinePoint::Point { x, y } => write_into_formatter(Some((format!("{x:?}"), format!("{y:?}"))), f),
      AffinePoint::Infinity => write_into_formatter(None, f),
    }
  }
}

impl<'a, T : GFArithmetic<'a>> fmt::Display for AffinePoint<T>
{
  fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result
  {
    match self
    {
      AffinePoint::Point { x, y } => write_into_formatter(Some((format!("{x}"), format!("{y}"))), f),
      AffinePoint::Infinity => write_into_formatter(None, f),
    }
  }
}

impl<'a, T : GFArithmetic<'a>> fmt::LowerHex for AffinePoint<T>
{
  fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result
  {
    match self
    {
      AffinePoint::Point { x, y } => write_into_formatter(
        Some((format!("{}", x.to_lower_hex_be()), format!("{}", y.to_lower_hex_be()))),
        f,
      ),
      AffinePoint::Infinity => write_into_formatter(None, f),
    }
  }
}

impl<'a, T : GFArithmetic<'a>> fmt::UpperHex for AffinePoint<T>
{
  fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result
  {
    match self
    {
      AffinePoint::Point { x, y } => write_into_formatter(
        Some((format!("{}", x.to_upper_hex_be()), format!("{}", y.to_upper_hex_be()))),
        f,
      ),
      AffinePoint::Infinity => write_into_formatter(None, f),
    }
  }
}

impl<'a, T : GFArithmetic<'a>> fmt::Binary for AffinePoint<T>
{
  fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result
  {
    match self
    {
      AffinePoint::Point { x, y } =>
      {
        write_into_formatter(Some((format!("{}", x.to_binary_be()), format!("{}", y.to_binary_be()))), f)
      }
      AffinePoint::Infinity => write_into_formatter(None, f),
    }
  }
}

fn write_into_formatter(point : Option<(String, String)>, f : &mut Formatter<'_>) -> fmt::Result
{
  match point
  {
    None => write!(f, "Infinity"),
    Some((x, y)) => write!(f, "Point {{ x: '{x}', y: '{y}' }}"),
  }
}
