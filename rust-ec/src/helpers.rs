use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand_core::CryptoRngCore;
use poly_algebra::gf::GFArithmetic;
use poly_algebra::helpers::solve_quadratic_equation_in_field;
use crate::affine_point::AffinePoint;
use crate::binary_ec::BinaryEC;

/// Function packs point according to the algorithm `6.9`.
pub fn pack_affine_point<'a, T : GFArithmetic<'a>>(point : &AffinePoint<T>) -> T
{
  match point
  {
    AffinePoint::Point { x: x_p, y: y_p } =>
    {
      if x_p.is_zero()
      {
        T::zero()
      }
      else
      {
        let x_inv = x_p.inverse();
        let y = y_p.clone() * x_inv;
        let i = y.trace();

        let x_p = x_p.get_value();
        if i.is_zero()
        {
          // i=0
          if (&x_p & BigUint::one()).is_one()
          {
            T::from_poly(x_p ^ BigUint::one())
          }
          else
          {
            T::from_poly(x_p)
          }
        }
        else
        {
          // i=1
          if (&x_p & BigUint::one()).is_zero()
          {
            T::from_poly(x_p ^ BigUint::one())
          }
          else
          {
            T::from_poly(x_p)
          }
        }
      }
    }
    AffinePoint::Infinity => T::zero(),
  }
}

/// Function unpacks compressed point from GF(2^m) field element according to the algorithm `6.10`.
pub fn unpack_affine_point<'a, T : GFArithmetic<'a>>(num : &T, ec : &BinaryEC<T>) -> AffinePoint<T>
{
  let mut x_p = num.get_value();
  if x_p.is_zero()
  {
    // Because prime poly has one bit more that ordinary number
    let power = num.get_prime_poly().bits() - 1 - 1;
    let power = BigUint::one() << power;
    return AffinePoint::Point {
      x : T::zero(),
      y : ec.b.pow(power),
    };
  }
  let k = {
    if (&x_p & BigUint::one()).is_one()
    {
      x_p ^= BigUint::one();
      BigUint::one()
    }
    else
    {
      BigUint::zero()
    }
  };
  let mut x_p_field = T::from(x_p.clone());
  if x_p_field.trace() != ec.a.as_biguint()
  {
    x_p ^= BigUint::one();
    x_p_field = T::from(x_p)
  }
  let w = {
    let x_p_2 = x_p_field.square();
    let x_p_3 = x_p_2.clone() * x_p_field.clone();
    let a = ec.a.as_field_el();
    x_p_3 + a * x_p_2 + ec.b.clone()
  };
  let x_p_inv_2 = {
    let x_p_inv = x_p_field.inverse();
    x_p_inv.square()
  };
  // Anyway we'll have 2 roots from the equation
  let z = solve_quadratic_equation_in_field::<T>(&T::one(), &(w * x_p_inv_2)).unwrap().0;
  let y_p = if z.trace() == k
  {
    z * x_p_field.clone()
  }
  else
  {
    (z + BigUint::one()) * x_p_field.clone()
  };
  AffinePoint::Point { x : x_p_field, y : y_p }
}

/// Function generates affine point by generating one point and solving quadratic equation.
pub fn generate_random_affine_point<'a, T : GFArithmetic<'a>>(rng : &mut impl CryptoRngCore, ec : &BinaryEC<T>)
  -> AffinePoint<T>
{
  loop
  {
    let u = T::rand(rng);
    let w = {
      let u_2 = u.square();
      let u_3 = u.clone() * u_2.clone();
      let a = ec.a.as_field_el();
      u_3 + a * u_2 + ec.b.clone()
    };
    if let Some((z, _)) = solve_quadratic_equation_in_field::<T>(&u, &w)
    {
      return AffinePoint::Point { x : u, y : z };
    }
  }
}
