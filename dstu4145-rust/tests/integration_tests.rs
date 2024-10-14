use num_bigint::BigUint;
use proptest::arbitrary::any;
use proptest::collection::vec;
use proptest::prelude::Strategy;

#[cfg(test)]
mod tests
{
  use proptest::arbitrary::any;
  use proptest::collection::vec;
  use proptest::prelude::Strategy;
  use rand_chacha::ChaCha20Rng;
  use rand_chacha::rand_core::{RngCore, SeedableRng};
  use sha3::Digest;
  use signature::{DigestSigner, DigestVerifier, RandomizedSigner, Signer, Verifier};
  use dstu4145_rust::sign::SigningKey;
  use rust_ec::binary_ec::BinaryEC;

  const PROP_TEST_BIGUINT_BYTE_LEN : usize = 128;
  fn arb_byte_vec() -> impl Strategy<Value = Vec<u8>> { vec(any::<u8>(), 0 .. PROP_TEST_BIGUINT_BYTE_LEN) }

  #[test]
  fn test1()
  {
    let pt = vec![0, 9];
    let mut rng = ChaCha20Rng::from_entropy();
    let ec = BinaryEC::generate_m431_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }

  // Signing with rng value
  #[test]
  fn sign_with_rng_verify_test_163()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m163_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_167()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m167_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_173()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m173_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_179()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m179_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_191()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m191_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_233()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m233_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_257()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m257_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_307()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m307_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_367()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m367_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_with_rng_verify_test_431()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m431_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_with_rng(&mut rng, &pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }

  // Ordinary signing
  #[test]
  fn sign_verify_test_163()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m163_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_167()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m167_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_173()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m173_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_179()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m179_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_191()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m191_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_233()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m233_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_257()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m257_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_307()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m307_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_367()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m367_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }
  #[test]
  fn sign_verify_test_431()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let ec = BinaryEC::generate_m431_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign(&pt);
    assert!(pub_key.verify(&pt, &signature).is_ok());
  }

  // Signing with custom digest
  #[test]
  fn sign_digest_verify_test_163()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m163_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    // Create figest for verifying
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_167()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m167_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_173()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m173_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_179()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m179_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_191()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m191_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_233()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m233_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_257()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m257_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_307()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m307_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_367()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m367_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
  #[test]
  fn sign_digest_verify_test_431()
  {
    let mut pt = vec![0; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill_bytes(&mut pt);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    let ec = BinaryEC::generate_m431_pb_curve();
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
    let signature = private_key.sign_digest(digest);
    let mut digest = sha3::Sha3_512::new();
    digest.update(&pt);
    assert!(pub_key.verify_digest(digest, &signature).is_ok());
  }
}
