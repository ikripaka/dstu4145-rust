use num_bigint::BigUint;
use num_traits::Zero;
use poly_algebra::gf::gf_def::GFArithmetic;
use poly_algebra::gf::gf_impl::{GF239, GF3};
use crate::binary_ec::BinaryEC3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AffinePoint
{
  Point
  {
    x : GF3,
    y : GF3,
  },
  Infinity,
}

impl AffinePoint
{
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

  pub fn neutral() -> AffinePoint { AffinePoint::Infinity }

  pub fn negative(&self) -> AffinePoint
  {
    match self
    {
      AffinePoint::Point { x, y } => AffinePoint::Point {
        x : x.clone(),
        y : x + y,
      },
      AffinePoint::Infinity => AffinePoint::Infinity,
    }
  }

  pub fn add(&self, ec : &BinaryEC3, q : &Self) -> Self
  {
    match self
    {
      AffinePoint::Point { x: x_p, y: y_p } => match q
      {
        AffinePoint::Point { x: x_q, y: y_q } =>
        {
          if *self == q.negative()
          {
            return AffinePoint::Infinity;
          }
          if self == q
          {
            return self.double(ec);
          }
          /// Calculating `x_r`
          let numerator = y_p + y_q;
          let denominator = (x_p + x_q).inverse();
          let fraction = &numerator * &denominator;
          let x_r = {
            let fraction2 = fraction.clone().square();
            &fraction + &fraction2 + x_p + x_q + GF3::from(ec.a.as_biguint())
          };
          /// Calculating `y_r`
          let y_r = {
            let tmp1 = x_p + &x_r;
            fraction * tmp1 + &x_r + y_p
          };
          AffinePoint::Point { x : x_r, y : y_r }
        }
        AffinePoint::Infinity => AffinePoint::Infinity,
      },
      AffinePoint::Infinity => q.clone(),
    }
  }

  pub fn double(&self, ec : &BinaryEC3) -> Self
  {
    match self
    {
      AffinePoint::Point { x: x_p, y: y_p } =>
      {
        let x_p_squared = x_p.clone().square();
        let x_r = {
          let x_p_squared_inv = x_p_squared.inverse();
          &x_p_squared + &(&ec.b * &x_p_squared_inv)
        };
        let y_r = {
          let x_p_inv = x_p.inverse();
          x_p_squared + &(x_p + &(y_p * &x_p_inv)) * &x_r + &x_r
        };
        AffinePoint::Point { x : x_r, y : y_r }
      }
      AffinePoint::Infinity => AffinePoint::Infinity,
    }
  }

  pub fn mul<N : Into<BigUint>>(&self, ec : &BinaryEC3, n : N) -> Self
  {
    let mut r = AffinePoint::neutral();
    let mut tmp = self.clone();
    // from LSB to MSB
    for x in n.into().to_str_radix(2).into_bytes().iter().rev()
    {
      if *x - b'0' == 1
      {
        r = r.add(ec, &tmp)
      }
      tmp = tmp.add(ec, &tmp)
    }
    r
  }
}
