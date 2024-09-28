use num_bigint::BigUint;
use num_traits::{ToBytes, Zero};
use rand_chacha::{ChaCha20Core, ChaCha20Rng};
use rand_chacha::rand_core::SeedableRng;
use signature::{DigestSigner, DigestVerifier, Error, RandomizedSigner, Signer, Verifier};
use signature::digest::Digest;
use signature::rand_core::CryptoRngCore;
use poly_algebra::gf::gf_def::GFArithmetic;
use poly_algebra::helpers::create_field_el_from_hash;
use rust_ec::affine_point::AffinePoint;
use rust_ec::binary_ec::BinaryEC;
use crate::error::Dstu4145Error;
use crate::helpers::{calculate_presign, transform_field_poly_into_number};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Signature
{
  r : Vec<u8>,
  s : Vec<u8>,
  l_d : u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VerifyingKey<T>
{
  ec : BinaryEC<T>,
  q : AffinePoint<T>,
  l_d : u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SigningKey<T>
{
  ec : BinaryEC<T>,
  d : T,
  l_d : u64,
}
impl<'a, T : GFArithmetic<'a>> Verifier<Signature> for VerifyingKey<T>
{
  fn verify(&self, msg : &[u8], signature : &Signature) -> Result<(), Error>
  {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    match verify(&self.ec, digest, &self.q, &signature.r, &signature.s)
    {
      Ok(_) => Ok(()),
      Err(e) =>
      {
        let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
        Err(Error::from(e))
      }
    }
  }
}

impl<'a, T : GFArithmetic<'a>, D : Digest> DigestVerifier<D, Signature> for VerifyingKey<T>
{
  fn verify_digest(&self, digest : D, signature : &Signature) -> Result<(), Error>
  {
    match verify(&self.ec, digest, &self.q, &signature.r, &signature.s)
    {
      Ok(_) => Ok(()),
      Err(e) =>
      {
        let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
        Err(Error::from(e))
      }
    }
  }
}
impl<'a, T : GFArithmetic<'a>> Signer<Signature> for SigningKey<T>
{
  fn try_sign(&self, msg : &[u8]) -> Result<Signature, Error>
  {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    match sign(&mut rng, &self.ec, digest, &self.d, self.l_d)
    {
      Ok(s) => Ok(s),
      Err(e) =>
      {
        let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
        Err(Error::from(e))
      }
    }
  }
}
impl<'a, T : GFArithmetic<'a>> RandomizedSigner<Signature> for SigningKey<T>
{
  fn try_sign_with_rng(&self, rng : &mut impl CryptoRngCore, msg : &[u8]) -> Result<Signature, Error>
  {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    match sign(rng, &self.ec, digest, &self.d, self.l_d)
    {
      Ok(s) => Ok(s),
      Err(e) =>
      {
        let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
        Err(Error::from(e))
      }
    }
  }
}

impl<'a, T : GFArithmetic<'a>, D : Digest> DigestSigner<D, Signature> for SigningKey<T>
{
  fn try_sign_digest(&self, digest : D) -> Result<Signature, Error>
  {
    let mut rng = ChaCha20Rng::from_entropy();
    match sign(&mut rng, &self.ec, digest, &self.d, self.l_d)
    {
      Ok(s) => Ok(s),
      Err(e) =>
      {
        let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
        Err(Error::from(e))
      }
    }
  }
}

impl<'a, T : GFArithmetic<'a>> SigningKey<T>
{
  pub fn generate(rng : &mut impl CryptoRngCore, ec : BinaryEC<T>, l_d : u64) -> crate::error::Result<(Self, VerifyingKey<T>)>
  {
    check_l_d_value(l_d, &ec)?;
    loop
    {
      let d = T::rand(rng);
      if !d.is_zero()
      {
        let q = ec.get_ref_bp().mul(&ec, d.clone()).negative();
        return Ok((Self { ec : ec.clone(), d, l_d }, VerifyingKey { ec, q, l_d }));
      }
    }
  }

  pub fn from_secret<B : AsRef<[u8]>>(ec : BinaryEC<T>, d : B, l_d : u64) -> crate::error::Result<Self>
  {
    check_l_d_value(l_d, &ec)?;
    let desired_length = T::get_m();
    let d = BigUint::from_bytes_be(d.as_ref());
    if d.bits() > desired_length as u64
    {
      return Err(Dstu4145Error::InvalidParamLength(
        desired_length as u64,
        d.bits(),
        "d".to_string(),
      ));
    }
    Ok(Self { ec, d : T::from(d), l_d })
  }

  pub fn verify_verifying_key(&self, verifying_key : &VerifyingKey<T>) -> crate::error::Result<()>
  {
    if self.ec != verifying_key.ec
    {
      return Err(Dstu4145Error::InvalidParams(format!(
        "Elliptic curves don't match, got EC with 'A in hex: {}, B in hex: {},  n in hex: {}, m: {}', \
        has to be EC with 'A in hex: {}, B in hex: {},  n in hex: {}, m: {}'",
        verifying_key.ec.get_a().as_field_el().to_upper_hex_be(),
        verifying_key.ec.get_b().to_upper_hex_be(),
        verifying_key.ec.get_ord().to_str_radix(16),
        T::get_m(),
        self.ec.get_a().as_field_el().to_upper_hex_be(),
        self.ec.get_b().to_upper_hex_be(),
        self.ec.get_ord().to_str_radix(16),
        T::get_m(),
      )));
    }
    if self.l_d != verifying_key.l_d
    {
      return Err(Dstu4145Error::InvalidParams(format!(
        "L_d parameters don't match, got: {}, has to be: {}",
        verifying_key.l_d, self.l_d,
      )));
    }
    let q_from_signing_key = self.ec.get_ref_bp().mul(&self.ec, self.d.clone()).negative();
    if q_from_signing_key != verifying_key.q
    {
      Err(Dstu4145Error::InvalidParams(
        "Invalid verifying key Q, check validity of params".to_string(),
      ))
    }
    else
    {
      Ok(())
    }
  }
}

pub fn sign<'a, T : GFArithmetic<'a>, D : Digest>(
  rng : &mut impl CryptoRngCore,
  ec : &BinaryEC<T>,
  digest : D,
  d : &T,
  l_d : u64,
) -> crate::error::Result<Signature>
{
  let hash = digest.finalize().to_vec();
  let h = create_field_el_from_hash::<T, _>(&hash);
  let (r, e) = loop
  {
    let (e, f_e) = calculate_presign(rng, ec);
    let y = h.clone() * f_e;
    let r = y.get_value();
    if !r.is_zero()
    {
      break (transform_field_poly_into_number(r, ec.get_ord()), e.get_value());
    }
  };
  let s = (e + d.get_value() * &r) % ec.get_ord();
  Ok(Signature {
    r : r.to_be_bytes().to_vec(),
    s : s.to_be_bytes().to_vec(),
    l_d,
  })
}

pub fn verify<'a, T : GFArithmetic<'a>, D : Digest>(
  ec : &BinaryEC<T>,
  digest : D,
  q : &AffinePoint<T>,
  r : &[u8],
  s : &[u8],
) -> crate::error::Result<()>
{
  //todo: maybe somehow check L_d?
  let hash = digest.finalize().to_vec();
  let h = create_field_el_from_hash::<T, _>(&hash);
  let r_original = BigUint::from_bytes_be(r);
  if r_original > *ec.get_ref_ord()
  {
    return Err(Dstu4145Error::InvalidParams(format!(
      "Invalid r parameter, got: {}, has to be less than n: {}",
      r_original.to_str_radix(16),
      ec.get_ord().to_str_radix(16)
    )));
  }
  let s_original = BigUint::from_bytes_be(s);
  if s_original > *ec.get_ref_ord()
  {
    return Err(Dstu4145Error::InvalidParams(format!(
      "Invalid s parameter, got: {}, has to be less than n: {}",
      s_original.to_str_radix(16),
      ec.get_ord().to_str_radix(16)
    )));
  }

  let r = {
    let s_p = ec.mul(ec.get_ref_bp(), s_original);
    let r_q = ec.mul(q, r_original.clone());
    ec.add(&s_p, &r_q)
  };
  match r
  {
    AffinePoint::Point { x: x_r, .. } =>
    {
      let y = h * x_r;
      let r_dash = transform_field_poly_into_number(y, ec.get_ord());
      if r_original == r_dash
      {
        Ok(())
      }
      else
      {
        Err(Dstu4145Error::IncorrectSignature)
      }
    }
    AffinePoint::Infinity => Err(Dstu4145Error::GotPointInInfinity),
  }
}

fn check_l_d_value<'a, T : GFArithmetic<'a>>(l_d : u64, ec : &BinaryEC<T>) -> crate::error::Result<()>
{
  if l_d % 16 != 0 || l_d < 2 * ec.get_ref_ord().bits()
  {
    return Err(Dstu4145Error::InvalidParams(format!(
      "Invalid L_d parameter, got: {l_d}, has to be a multiple of 16 & >= than 2*{}",
      ec.get_ref_ord().bits()
    )));
  }
  Ok(())
}
