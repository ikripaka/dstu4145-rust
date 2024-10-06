use dstu4145_rust::sign::{Signature, SigningKey, VerifyingKey};
use poly_algebra::gf::{GFArithmetic};
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::{CryptoRngCore, SeedableRng};
use rust_ec::binary_ec::BinaryEC;
use sha3::Digest;
use signature::{DigestSigner, DigestVerifier, RandomizedSigner, Verifier};

fn main() -> dstu4145_rust::error::Result<()>
{
  let msg = "hello world!".as_bytes();
  let mut rng = ChaCha20Rng::from_entropy();
  let ec = BinaryEC::generate_m167_pb_curve();

  let (signature_1, _priv_key_1, pub_key_1) = sign_ordinary(msg, &mut rng, &ec);
  let (signature_2, _priv_key_2, pub_key_2) = sign_rng(msg, &mut rng, &ec);
  let (signature_3, _priv_key_3, pub_key_3) = {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    sign_digest(digest, &mut rng, &ec)
  };
  assert!(pub_key_1.verify(msg, &signature_1).is_ok());
  assert!(pub_key_2.verify(msg, &signature_2).is_ok());
  {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    assert!(pub_key_3.verify_digest(digest, &signature_3).is_ok());
  }

  assert!(pub_key_1.verify(msg, &signature_2).is_err());
  assert!(pub_key_1.verify(msg, &signature_3).is_err());
  assert!(pub_key_2.verify(msg, &signature_1).is_err());
  assert!(pub_key_2.verify(msg, &signature_3).is_err());

  {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    assert!(pub_key_3.verify_digest(digest, &signature_1).is_err());
  }
  {
    let mut digest = sha3::Sha3_512::new();
    digest.update(msg);
    assert!(pub_key_3.verify_digest(digest, &signature_2).is_err());
  }
  Ok(())
}

fn sign_ordinary<'a, T : GFArithmetic<'a>>(
  msg : &[u8],
  rng : &mut impl CryptoRngCore,
  ec : &BinaryEC<T>,
) -> (Signature, SigningKey<T>, VerifyingKey<T>)
{
  let (private_key, pub_key) = SigningKey::generate(rng, ec.clone(), 1024).unwrap();
  (private_key.sign_with_rng(rng, msg), private_key, pub_key)
}
fn sign_rng<'a, T : GFArithmetic<'a>>(
  msg : &[u8],
  rng : &mut impl CryptoRngCore,
  ec : &BinaryEC<T>,
) -> (Signature, SigningKey<T>, VerifyingKey<T>)
{
  let (private_key, pub_key) = SigningKey::generate(rng, ec.clone(), 1024).unwrap();
  let signature = private_key.sign_with_rng(rng, msg);
  (private_key.sign_with_rng(rng, msg), private_key, pub_key)
}
fn sign_digest<'a, T : GFArithmetic<'a>, D : Digest>(
  digest : D,
  rng : &mut impl CryptoRngCore,
  ec : &BinaryEC<T>,
) -> (Signature, SigningKey<T>, VerifyingKey<T>)
{
  let (private_key, pub_key) = SigningKey::generate(rng, ec.clone(), 1024).unwrap();
  (private_key.sign_digest(digest), private_key, pub_key)
}
