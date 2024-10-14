use num_bigint::BigUint;
use num_traits::{ToBytes, Zero};
use rand_chacha::{ChaCha20Rng};
use rand_chacha::rand_core::SeedableRng;
use signature::{DigestSigner, DigestVerifier, Error, RandomizedSigner, Signer, Verifier};
use signature::digest::Digest;
use signature::rand_core::CryptoRngCore;
use bytes::{BytesMut};
use poly_algebra::gf::GFArithmetic;
use poly_algebra::helpers::{create_field_el_from_hash, generate_num};
use rust_ec::affine_point::AffinePoint;
use rust_ec::binary_ec::BinaryEC;
use crate::error::Dstu4145Error;
use crate::helpers::{calculate_presign, check_public_key_correctness, transform_field_poly_into_number};

/// Struct saves signature info as output structure.
/// It contains in itself $(r, s)$ values with $L_d$ parameter.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Signature
{
  r : Vec<u8>,
  s : Vec<u8>,
  l_d : u64,
}

/// Struct that characterize __Private key__ for making digital signature.
/// It consists of saved EC, affine point $Q$ on the EC, $L_d$ parameter.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VerifyingKey<T>
{
  ec : BinaryEC<T>,
  q : AffinePoint<T>,
  l_d : u64,
}

/// Struct designed for restoring Verifying key from packed EC point
/// It consists from packed value of affine point in the GF element, $L_d$ parameter and EC.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VerifyingKeyConstructor<T>
{
  pub ec : BinaryEC<T>,
  pub q : T,
  pub l_d : u64,
}

/// Struct that characterize __Public key__ for checking digital signature.
/// It consists from secred $d$ value with $L(n) - 1$ bit len, $L_d$ parameter and EC.
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
    if signature.l_d != self.l_d
    {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(Dstu4145Error::InvalidParams(format!(
        "Unequal L_D param in both structs, VerifyingKey.l_d: {}, Signature.l_d: {}",
        self.l_d, signature.l_d
      )));
      return Err(Error::from(e));
    }
    verify(&self.ec, digest, &self.q, &signature.r, &signature.s, signature.l_d).map_err(|e| {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
      Error::from(e)
    })
  }
}

impl<'a, T : GFArithmetic<'a>, D : Digest> DigestVerifier<D, Signature> for VerifyingKey<T>
{
  fn verify_digest(&self, digest : D, signature : &Signature) -> Result<(), Error>
  {
    if signature.l_d != self.l_d
    {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(Dstu4145Error::InvalidParams(format!(
        "Unequal L_D param in both structs, VerifyingKey.l_d: {}, Signature.l_d: {}",
        self.l_d, signature.l_d
      )));
      return Err(Error::from(e));
    }
    verify(&self.ec, digest, &self.q, &signature.r, &signature.s, signature.l_d).map_err(|e| {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
      Error::from(e)
    })
  }
}
impl<'a, T : GFArithmetic<'a>> Signer<Signature> for SigningKey<T>
{
  fn try_sign(&self, msg : &[u8]) -> Result<Signature, Error>
  {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    sign(&mut rng, &self.ec, digest, &self.d, self.l_d).map_err(|e| {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
      Error::from(e)
    })
  }
}
impl<'a, T : GFArithmetic<'a>> RandomizedSigner<Signature> for SigningKey<T>
{
  fn try_sign_with_rng(&self, rng : &mut impl CryptoRngCore, msg : &[u8]) -> Result<Signature, Error>
  {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    sign(rng, &self.ec, digest, &self.d, self.l_d).map_err(|e| {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
      Error::from(e)
    })
  }
}

impl<'a, T : GFArithmetic<'a>, D : Digest> DigestSigner<D, Signature> for SigningKey<T>
{
  fn try_sign_digest(&self, digest : D) -> Result<Signature, Error>
  {
    let mut rng = ChaCha20Rng::from_entropy();
    sign(&mut rng, &self.ec, digest, &self.d, self.l_d).map_err(|e| {
      let e : Box<(dyn std::error::Error + Send + Sync + 'static)> = Box::new(e);
      Error::from(e)
    })
  }
}

impl<'a, T : GFArithmetic<'a>> SigningKey<T>
{
  /// Function gets a copy of private key. Be careful in using of this command!
  pub fn get_private_key(&self) -> BigUint { self.d.clone() }

  /// Function generates __Private key__ from PRNG.
  pub fn generate(rng : &mut impl CryptoRngCore, ec : BinaryEC<T>, l_d : u64) -> crate::error::Result<(Self, VerifyingKey<T>)>
  {
    check_l_d_value(l_d, &ec)?;
    loop
    {
      let d = generate_num(rng, ec.get_ref_ord().bits() - 1);
      if !d.is_zero()
      {
        let q = ec.get_ref_bp().mul(&ec, d.clone()).negative();
        return Ok((Self { ec : ec.clone(), d, l_d }, VerifyingKey { ec, q, l_d }));
      }
    }
  }

  /// Function creates __Private key__ from given values that may be pregenerated.
  pub fn from_secret<B : AsRef<[u8]>>(ec : BinaryEC<T>, d : B, l_d : u64) -> crate::error::Result<(Self, VerifyingKey<T>)>
  {
    check_l_d_value(l_d, &ec)?;
    let desired_length = ec.get_ref_ord().bits() - 1;
    let d = BigUint::from_bytes_be(d.as_ref());
    if d.bits() > desired_length
    {
      return Err(Dstu4145Error::InvalidParamLength(desired_length, d.bits(), "d".to_string()));
    }
    let q = ec.get_ref_bp().mul(&ec, d.clone()).negative();
    Ok((Self { ec : ec.clone(), d, l_d }, VerifyingKey { ec, q, l_d }))
  }

  /// Function checks on correctness __Public key__ on correctness. To reduce amount of incorrect signature.
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
  /// Function packs signature into bytes according to the algorithm `5.10`.
  pub fn pack(&self) -> Vec<u8>
  {
    let element_size = ((self.l_d >> 1) / 8) as usize;
    let mut buf = BytesMut::zeroed(element_size - self.s.len());
    buf.extend(&self.s);
    buf.extend(&BytesMut::zeroed(element_size - self.r.len()));
    buf.extend(&self.r);
    buf.to_vec()
  }

  pub fn get_r(&self) -> Vec<u8> { self.r.clone() }

  pub fn get_ref_r(&self) -> &[u8] { &self.r }

  pub fn get_s(&self) -> Vec<u8> { self.s.clone() }

  pub fn get_ref_s(&self) -> &[u8] { &self.s }

  pub fn get_l_d(&self) -> u64 { self.l_d }
}
impl TryFrom<&[u8]> for Signature
{
  type Error = Dstu4145Error;

  fn try_from(value : &[u8]) -> Result<Self, Self::Error>
  {
    let mut buf = BytesMut::from(value);
    let l_d = buf.len();
    let l_d_bits = l_d * 8;
    if l_d_bits % 16 != 0
    {
      return Err(Dstu4145Error::InvalidLDLength(
        "Length has to be divisible onto 16".to_string(),
        l_d_bits as u64,
      ));
    }
    let s = buf.split_to(l_d >> 1);
    Ok(Signature {
      r : buf.to_vec(),
      s : s.to_vec(),
      l_d : l_d_bits as u64,
    })
  }
}

/// This conversion gets as the input hex string that is divisible on 4 (in context of symbol length)
impl TryFrom<&str> for Signature
{
  type Error = Dstu4145Error;

  fn try_from(value : &str) -> Result<Self, Self::Error>
  {
    let len = value.chars().count();
    if len % 4 != 0
    {
      return Err(Dstu4145Error::InvalidLDLength(
        "has to be divisible on 16".to_string(),
        len as u64 * 8,
      ));
    }
    let bytes = value.as_bytes()
            .chunks(2) // Take two characters at a time
            .map(|chunk| {
                let hex_str = std::str::from_utf8(chunk).unwrap(); // Convert to &str
                u8::from_str_radix(hex_str, 16).unwrap() // Parse as base-16
            })
            .collect::<Vec<u8>>();
    Signature::try_from(bytes.as_slice())
  }
}

impl<'a, T : GFArithmetic<'a>> VerifyingKey<T>
{
  /// Function returns as output - packed public key `Q` into bytes.
  pub fn pack(&self) -> Vec<u8> { self.q.pack().get_value().to_bytes_be() }

  pub fn get_pub_key(&self) -> AffinePoint<T> { self.q.clone() }
}

impl<'a, T : GFArithmetic<'a>> TryFrom<VerifyingKeyConstructor<T>> for VerifyingKey<T>
{
  type Error = Dstu4145Error;

  fn try_from(value : VerifyingKeyConstructor<T>) -> Result<Self, Self::Error>
  {
    check_l_d_value(value.l_d, &value.ec)?;
    let q = check_public_key_correctness(&value.ec, &value.q)?;
    Ok(VerifyingKey {
      ec : value.ec,
      q,
      l_d : value.l_d,
    })
  }
}

/// Function performs creating of the signature to the specific message.
fn sign<'a, T : GFArithmetic<'a>, D : Digest>(
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
      break (transform_field_poly_into_number(r, ec.get_ord()), e);
    }
  };
  sign_inner(ec, &r, &e, d, l_d)
}

/// Function performs last step of signing.
fn sign_inner<'a, T : GFArithmetic<'a>>(
  ec : &BinaryEC<T>,
  r : &BigUint,
  e : &BigUint,
  d : &BigUint,
  l_d : u64,
) -> crate::error::Result<Signature>
{
  let s = (e.clone() + d * r) % ec.get_ord();
  Ok(Signature {
    r : r.to_be_bytes().to_vec(),
    s : s.to_be_bytes().to_vec(),
    l_d,
  })
}

/// Function verifies whether the given signature is correct.
fn verify<'a, T : GFArithmetic<'a>, D : Digest>(
  ec : &BinaryEC<T>,
  digest : D,
  q : &AffinePoint<T>,
  r : &[u8],
  s : &[u8],
  l_d : u64,
) -> crate::error::Result<()>
{
  check_l_d_value(l_d, ec)?;
  let hash = digest.finalize().to_vec();
  let h = create_field_el_from_hash::<T, _>(hash);
  // Check `r`, `s` validity
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

/// Function performs last step of verifying signature.
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

/// Function checks $L_D$ value to conform next properties:
/// * $L_D$ has to be divisible into 16;
/// * $L_D$ has to be >= than $L_n$ value.
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
mod tests_from_docs
{
  use num_bigint::BigUint;
  use num_traits::Num;
  use signature::{Signer, Verifier};
  use poly_algebra::gf::{GFArithmetic, GF163};
  use poly_algebra::helpers::{create_field_el_from_hash, get_string_array_plain};
  use rust_ec::affine_point::AffinePoint;
  use rust_ec::binary_ec::BinaryEC;
  use crate::error::Dstu4145Error;
  use crate::helpers::{calculate_presign, transform_field_poly_into_number};
  use crate::sign::{sign_inner, verify_inner, SigningKey};

  #[test]
  fn test_from_doc1() -> crate::error::Result<()>
  {
    let mut ec = BinaryEC::generate_m163_pb_curve_from_examples();
    let msg = "hello";

    // Signing
    let (private_key, pub_key) = SigningKey::from_secret(
      ec.clone(),
      hex_literal::hex!("0183F60FDF7951FF47D67193F8D073790C1C9B5A3E"),
      512,
    )?;
    assert_eq!(
      ec.get_bp(),
      AffinePoint::Point {
        x : GF163::from_hex_be("072D867F93A93AC27DF9FF01AFFE74885C8C540420")?,
        y : GF163::from_hex_be("00224A9C3947852B97C5599D5F4AB81122ADC3FD9B")?
      }
    );
    assert_eq!(
      pub_key.q,
      AffinePoint::Point {
        x : GF163::from_hex_be("57DE7FDE023FF929CB6AC785CE4B79CF64ABDC2DA")?,
        y : GF163::from_hex_be("3E85444324BCF06AD85ABF6AD7B5F34770532B9AA")?
      }
    );
    let hash = hex_literal::hex!("003A2EB95B7180166DDF73532EEB76EDAEF52247FF").to_vec();
    let h = create_field_el_from_hash::<GF163, _>(hash);
    assert_eq!(h, GF163::from_hex_be("03A2EB95B7180166DDF73532EEB76EDAEF52247FF")?);
    let e = BigUint::from_bytes_be(&hex_literal::hex!("01025E40BD97DB012B7A1D79DE8E12932D247F61C6"));
    let ep = ec.mul(ec.get_ref_bp(), e.clone());
    assert_eq!(
      ep,
      AffinePoint::Point {
        x : GF163::from_hex_be("042A7D756D70E1C9BA62D2CB43707C35204EF3C67C")?,
        y : GF163::from_hex_be("05310AE5E560464A95DC80286F17EB762EC544B15B")?
      }
    );
    let sign = if let AffinePoint::Point { x: x_p, .. } = ep
    {
      let y = h.clone() * x_p;
      assert_eq!(y, GF163::from_hex_be("0274EA2C0CAA014A0D80A424F59ADE7A93068D08A7")?);
      let r = transform_field_poly_into_number(y, ec.get_ord());
      let sign = sign_inner(&ec, &r, &e, &private_key.d, private_key.l_d)?;
      assert_eq!(
        sign.r,
        hex_literal::hex!("0274EA2C0CAA014A0D80A424F59ADE7A93068D08A7").to_vec()
      );
      assert_eq!(
        sign.s,
        hex_literal::hex!("02100D86957331832B8E8C230F5BD6A332B3615ACA").to_vec()
      );
      sign
    }
    else
    {
      return Err(Dstu4145Error::GotPointInInfinity);
    };
    sign.pack();

    // Verifying
    let r_original = BigUint::from_bytes_be(&sign.r);
    let s_original = BigUint::from_bytes_be(&sign.s);
    assert_eq!(
      sign.r,
      hex_literal::hex!("0274EA2C0CAA014A0D80A424F59ADE7A93068D08A7").to_vec()
    );
    assert_eq!(
      sign.s,
      hex_literal::hex!("02100D86957331832B8E8C230F5BD6A332B3615ACA").to_vec()
    );
    let r = {
      let s_p = ec.mul(ec.get_ref_bp(), s_original.clone());
      let r_q = ec.mul(&pub_key.q, r_original.clone());
      ec.add(&s_p, &r_q)
    };
    assert_eq!(
      r,
      AffinePoint::Point {
        x : GF163::from_hex_be("042A7D756D70E1C9BA62D2CB43707C35204EF3C67C")?,
        y : GF163::from_hex_be("05310AE5E560464A95DC80286F17EB762EC544B15B")?
      }
    );
    match r
    {
      AffinePoint::Point { x: x_r, .. } =>
      {
        assert_eq!(x_r, GF163::from_hex_be("042A7D756D70E1C9BA62D2CB43707C35204EF3C67C")?);
        let y = h.clone() * x_r;
        assert_eq!(y, GF163::from_hex_be("0274EA2C0CAA014A0D80A424F59ADE7A93068D08A7")?);
        let r_dash = transform_field_poly_into_number(y, ec.get_ord());
        assert_eq!(
          r_dash,
          BigUint::from_str_radix("0274EA2C0CAA014A0D80A424F59ADE7A93068D08A7", 16)?
        );
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
