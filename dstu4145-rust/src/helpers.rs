use num_bigint::BigUint;
use num_traits::One;
use signature::digest::Digest;
use signature::rand_core::CryptoRngCore;
use poly_algebra::gf::GFArithmetic;
use poly_algebra::helpers::generate_num;
use rust_ec::affine_point::AffinePoint;
use rust_ec::binary_ec::BinaryEC;
use crate::sign::Signature;

/// Function calculates presign and returns tuple `(e, F_e)`
pub fn calculate_presign<'a, T : GFArithmetic<'a>>(rng : &mut impl CryptoRngCore, ec : &BinaryEC<T>) -> (BigUint, T)
{
  loop
  {
    let e = generate_num(rng, ec.get_ref_ord().bits() - 1);
    let r = ec.get_ref_bp().mul(ec, e.clone());
    if let AffinePoint::Point { x: x_p, .. } = r
    {
      if !x_p.is_zero()
      {
        return (e, x_p);
      }
    }
  }
}

pub fn transform_field_poly_into_number<T1 : Into<BigUint>, T2 : Into<BigUint>>(poly : T1, n : T2) -> BigUint
{
  let mask = (BigUint::one() << (n.into().bits() - 1)) - BigUint::one();
  poly.into() & mask
}
