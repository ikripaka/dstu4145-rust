use num_bigint::BigUint;
use num_traits::{ToBytes, Zero};
use rand_chacha::{ChaCha20Core, ChaCha20Rng};
use rand_chacha::rand_core::SeedableRng;
use signature::{DigestSigner, DigestVerifier, Error, RandomizedSigner, Signer, Verifier};
use signature::digest::Digest;
use signature::rand_core::CryptoRngCore;
use poly_algebra::gf::gf_def::GFArithmetic;
use poly_algebra::helpers::{create_field_el_from_hash, generate_num};
use rust_ec::affine_point::AffinePoint;
use rust_ec::binary_ec::BinaryEC;
use crate::error::Dstu4145Error;
use crate::helpers::{calculate_presign, generate_d, transform_field_poly_into_number};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Signature
{
  r : Vec<u8>,
  s : Vec<u8>,
  l_d : u64,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VerifyingKey<T>
{
  ec : BinaryEC<T>,
  q : AffinePoint<T>,
  l_d : u64,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SigningKey<T>
{
  ec : BinaryEC<T>,
  d : BigUint,
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
      let d = generate_d(rng, ec.get_ord());
      if !d.is_zero()
      {
        let q = ec.get_ref_bp().mul(&ec, d.clone()).negative();
        return Ok((Self { ec : ec.clone(), d, l_d }, VerifyingKey { ec, q, l_d }));
      }
    }
  }

  pub fn from_secret<B : AsRef<[u8]>>(ec : BinaryEC<T>, d : B, l_d : u64) -> crate::error::Result<(Self, VerifyingKey<T>)>
  {
    check_l_d_value(l_d, &ec)?;
    let desired_length = ec.get_ref_ord().bits();
    let d = BigUint::from_bytes_be(d.as_ref());
    if d.bits() > desired_length
    {
      return Err(Dstu4145Error::InvalidParamLength(desired_length, d.bits(), "d".to_string()));
      //    49FF09C848613AEA23699F78C960D5174617311ADCC
    }
    println!("d: {:X}, bp: {:X}", d, ec.get_bp());
    let q = ec.get_ref_bp().mul(&ec, d.clone()).negative();
    Ok((Self { ec : ec.clone(), d, l_d }, VerifyingKey { ec, q, l_d }))
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

impl Signature
{
  pub fn pack(&self) -> Vec<u8>
  {
    // todo!();
    vec![]
  }
}

pub fn sign<'a, T : GFArithmetic<'a>, D : Digest>(
  rng : &mut impl CryptoRngCore,
  ec : &BinaryEC<T>,
  digest : D,
  d : &BigUint,
  l_d : u64,
) -> crate::error::Result<Signature>
{
  let hash = digest.finalize().to_vec();
  let h = create_field_el_from_hash::<T, _>(hash);
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
  sign_inner(ec, &r, &e, d, l_d)
}

fn sign_inner<'a, T : GFArithmetic<'a>>(
  ec : &BinaryEC<T>,
  r : &BigUint,
  e : &BigUint,
  d : &BigUint,
  l_d : u64,
) -> crate::error::Result<Signature>
{
  println!(
    "e: {}, \n d: {},\n r: {},\n n: {}",
    e.to_str_radix(16),
    d.to_str_radix(16),
    r.to_str_radix(16),
    ec.get_ord().to_str_radix(16)
  );
  let s = (e.clone() + d * r) % ec.get_ord();
  println!("r: {}, s: {}", r.to_str_radix(16), s.to_str_radix(16));
  Ok(Signature {
    r : r.to_be_bytes().to_vec(),
    s : s.to_be_bytes().to_vec(),
    l_d,
  })
}
// 400000000000000000002bec12be2262d39bcf14d
// 3014648760584073996496740520455678628842031897290

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
  let h = create_field_el_from_hash::<T, _>(hash);
  /// Check `r`, `s` validity
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
  verify_inner(ec, q, &h, &r_original, &s_original)
}

fn verify_inner<'a, T : GFArithmetic<'a>>(
  ec : &BinaryEC<T>,
  q : &AffinePoint<T>,
  h : &T,
  r_original : &BigUint,
  s_original : &BigUint,
) -> crate::error::Result<()>
{
  let r = {
    let s_p = ec.mul(ec.get_ref_bp(), s_original.clone());
    let r_q = ec.mul(q, r_original.clone());
    ec.add(&s_p, &r_q)
  };
  match r
  {
    AffinePoint::Point { x: x_r, .. } =>
    {
      let y = h.clone() * x_r;
      let r_dash = transform_field_poly_into_number(y, ec.get_ord());
      if *r_original == r_dash
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

#[cfg(test)]
mod tests
{
  use num_bigint::BigUint;
  use rand_chacha::ChaCha20Rng;
  use rand_chacha::rand_core::SeedableRng;
  use signature::{Signer, Verifier};
  use poly_algebra::gf::gf_def::GF163;
  use poly_algebra::helpers::create_field_el_from_hash;
  use rust_ec::affine_point::AffinePoint;
  use rust_ec::binary_ec::BinaryEC;
  use crate::error::Dstu4145Error;
  use crate::helpers::{calculate_presign, transform_field_poly_into_number};
  use crate::sign::{sign_inner, verify_inner, SigningKey};

  #[test]
  fn test_from_doc1() -> crate::error::Result<()>
  {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut ec = BinaryEC::generate_m163_pb_curve_from_examples();
    let msg = "hello";

    // Signing
    let (private_key, pub_key) = SigningKey::from_secret(
      ec.clone(),
      hex_literal::hex!("0183F60FDF7951FF47D67193F8D073790C1C9B5A3E"),
      512,
    )?;
    println!("q: {:X}", pub_key.q);
    let hash = hex_literal::hex!("09C9C44277910C9AAEE486883A2EB95B7180166DDF73532EEB76EDAEF52247FF").to_vec();
    let h = create_field_el_from_hash::<GF163, _>(hash);
    println!("h: {:X}", h);
    let e = BigUint::from_bytes_be(&hex_literal::hex!("01025E40BD97DB012B7A1D79DE8E12932D247F61C6"));
    let ep = ec.mul(ec.get_ref_bp(), e.clone());
    println!("eP: {:X}", ep);
    let sign = if let AffinePoint::Point { x: x_p, .. } = ep
    {
      let y = h.clone() * x_p;
      println!("y: {:X}", y);
      let r = transform_field_poly_into_number(y, ec.get_ord());
      let sign = sign_inner(&ec, &r, &e, &private_key.d, private_key.l_d)?;
      println!("r: {:X?}, s: {:X?}", sign.r, sign.s);
      sign
    }
    else
    {
      return Err(Dstu4145Error::GotPointInInfinity);
    };
    sign.pack();

    // Verifying
    println!("verifying ");
    let r_original = BigUint::from_bytes_be(&sign.r);
    let s_original = BigUint::from_bytes_be(&sign.s);
    println!("r: {}, s: {}", s_original.to_str_radix(16), s_original.to_str_radix(16));
    let r = {
      let s_p = ec.mul(ec.get_ref_bp(), s_original.clone());
      let r_q = ec.mul(&pub_key.q, r_original.clone());
      ec.add(&s_p, &r_q)
    };
    println!("R: {:X}", r);
    match r
    {
      AffinePoint::Point { x: x_r, .. } =>
      {
        println!("x_R: {:X}, h: {:X}", x_r, h);
        let y = h.clone() * x_r;
        println!("y: {:X}", h);

        let r_dash = transform_field_poly_into_number(y, ec.get_ord());
        println!("r': {}, r: {}", r_dash.to_str_radix(16), r_original.to_str_radix(16));

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
    }?;

    Ok(())
  }
}
