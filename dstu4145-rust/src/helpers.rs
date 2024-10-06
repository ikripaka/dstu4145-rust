use num_bigint::BigUint;
use num_traits::One;
use signature::rand_core::CryptoRngCore;
use poly_algebra::gf::GFArithmetic;
use poly_algebra::helpers::generate_num;
use rust_ec::affine_point::AffinePoint;
use rust_ec::binary_ec::BinaryEC;
use rust_ec::helpers::unpack_affine_point;
use crate::error::Dstu4145Error;

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

/// Function transforms field poly number into ordinary one by truncating poly to $L_n - 1$ bits.
pub fn transform_field_poly_into_number<T1 : Into<BigUint>, T2 : Into<BigUint>>(poly : T1, n : T2) -> BigUint
{
  let mask = (BigUint::one() << (n.into().bits() - 1)) - BigUint::one();
  poly.into() & mask
}

/// Function checks correctness of the public key according to the algorithm `10.1`.
pub fn check_public_key_correctness<'a, T : GFArithmetic<'a>>(
  ec : &BinaryEC<T>,
  compressed_p : &T,
) -> crate::error::Result<AffinePoint<T>>
{
  let decompressed_point = unpack_affine_point(compressed_p, ec);
  if decompressed_point == AffinePoint::Infinity
  {
    return Err(Dstu4145Error::FailedPublicKeyCheck(
      "public key can't be a point at infinity".to_string(),
    ));
  }
  if !ec.check_affine_point(&decompressed_point)
  {
    return Err(Dstu4145Error::FailedPublicKeyCheck(format!(
      "{decompressed_point:X} doesn't belong to the curve"
    )));
  }
  if ec.mul(&decompressed_point, ec.get_ord()) != AffinePoint::Infinity
  {
    return Err(Dstu4145Error::FailedPublicKeyCheck(format!(
      "incorrect order, order of Q:'{decompressed_point:X}' != {:X}",
      ec.get_ord()
    )));
  }
  Ok(decompressed_point)
}
