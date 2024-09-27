use getset::Getters;
use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use poly_algebra::gf::gf_def::GFArithmetic;
use poly_algebra::gf::gf_impl::GF3;
use crate::affine_point::AffinePoint;
use crate::helpers::PreGeneratedParams;
#[derive(Getters)]
#[getset(get)]
pub struct BinaryEC3
{
  //todo: add generic parameter
  pub(crate) a : ACoefficient,
  pub(crate) b : GF3,
  pub(crate) bp : AffinePoint,
  pub(crate) n : BigUint,
}

// todo: implement builder pattern

#[derive(Clone, Debug)]
pub enum ACoefficient
{
  Zero,
  One,
}
impl ACoefficient
{
  pub fn as_biguint(&self) -> BigUint
  {
    match self
    {
      ACoefficient::Zero => BigUint::zero(),
      ACoefficient::One => BigUint::one(),
    }
  }
  // todo: implement for generic type T
}

impl From<PreGeneratedParams> for BinaryEC3
{
  fn from(value : PreGeneratedParams) -> Self { value.generate_curve() }
}

impl BinaryEC3
{
  // y^2 * z + xyz = x^3 + Ax^3 * z + Bz^3
  // y^2 + xy = x^3 + Ax^2 + B
  /// Function checks whether point belongs to the specified curve
  pub fn check_affine_point(&self, point : AffinePoint) -> bool
  {
    match point
    {
      AffinePoint::Point { x: x_p, y: y_p } =>
      {
        y_p.square() + &x_p * &y_p == &x_p.pow(3_u8) + &(&GF3::from(self.a.as_biguint()) * &x_p.square()) + &self.b
      }
      AffinePoint::Infinity => false,
    }
  }

  pub fn add(&self, point1 : &AffinePoint, point2 : &AffinePoint) -> AffinePoint { point1.add(self, point2) }

  pub fn double(&self, point : &AffinePoint) -> AffinePoint { point.double(self) }

  pub fn mul<N : Into<BigUint>>(&self, point : &AffinePoint, n : N) -> AffinePoint { point.mul(self, n) }
}
