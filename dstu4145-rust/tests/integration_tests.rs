#[cfg(test)]
mod tests
{
  use rand_chacha::ChaCha20Rng;
  use rand_chacha::rand_core::SeedableRng;
  use signature::{Signer, Verifier};
  use dstu4145_rust::sign::SigningKey;
  use rust_ec::binary_ec::BinaryEC;

  #[test]
  fn test1() -> dstu4145_rust::error::Result<()>
  {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut ec = BinaryEC::generate_m431_pb_curve();
    let msg = "hello";
    let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 2048)?;
    let signature = private_key.sign(msg.as_bytes());
    let res = pub_key.verify(msg.as_bytes(), &signature)?;
    Ok(())
  }
}
