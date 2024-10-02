#[cfg(test)]
mod property_tests
{

  use proptest::arbitrary::any;
  use proptest::collection::vec;
  use proptest::prelude::Strategy;
  use proptest::proptest;
  use rand_chacha::ChaCha20Rng;
  use rand_chacha::rand_core::SeedableRng;
  use sha3::Digest;
  use signature::{DigestSigner, DigestVerifier, RandomizedSigner, Signer, Verifier};
  use dstu4145_rust::sign::SigningKey;
  use rust_ec::binary_ec::BinaryEC;
  const PROP_TEST_BIGUINT_BYTE_LEN : usize = 128;
  fn arb_byte_vec() -> impl Strategy<Value = Vec<u8>> { vec(any::<u8>(), 0 .. PROP_TEST_BIGUINT_BYTE_LEN) }

  // Disclaimer: these tests are a little bit slow, you have to wait for a really long time to finish them.

  // Signing with rng value
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_163(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m163_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_167(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m167_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_173(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m173_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_179(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m179_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_191(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m191_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_233(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m233_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_257(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m257_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_307(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m307_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_367(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m367_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_with_rng_verify_test_431(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m431_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_with_rng(&mut rng, &pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }

  // Ordinary signing
  proptest! {
      #[test]
      fn sign_verify_test_163(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m163_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_167(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m167_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_173(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m173_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_179(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m179_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_191(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m191_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_233(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m233_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_257(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m257_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_307(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m307_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_367(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m367_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }
  proptest! {
      #[test]
      fn sign_verify_test_431(pt in arb_byte_vec()) {
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m431_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign(&pt);
        assert!(pub_key.verify(&pt, &signature).is_ok());
      }
  }

  // Signing with custom digest
  proptest! {
      #[test]
      fn sign_digest_verify_test_163(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m163_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_167(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m167_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_173(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m173_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_179(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m179_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_191(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m191_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_233(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m233_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_257(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m257_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_307(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m307_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_367(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m367_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
  proptest! {
      #[test]
      fn sign_digest_verify_test_431(pt in arb_byte_vec()) {
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        let mut rng = ChaCha20Rng::from_entropy();
        let ec = BinaryEC::generate_m431_pb_curve();
        let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048).unwrap();
        let signature = private_key.sign_digest(digest);
        let mut digest = sha3::Sha3_512::new();
        digest.update(&pt);
        assert!(pub_key.verify_digest(digest, &signature).is_ok());
    }
  }
}
