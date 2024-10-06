use num_bigint::ParseBigIntError;
use num_traits::{One, Zero};
use poly_algebra::gf::{GFArithmetic, GFGetters, GF257, GF367};
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;

fn main() -> Result<(), ParseBigIntError>
{
  let mut rng = ChaCha20Rng::from_entropy();
  let a = GF257::rand(&mut rng);
  let b = GF367::from_poly(a.get_value());
  let b_dash = GF367::from_hex_be(format!("{:X}", a))?;
  let x_rand = GF367::rand(&mut rng);
  assert_eq!(b, b_dash);
  assert_ne!(GF367::from_poly(a.inverse()), b.inverse());
  assert_eq!(&a * &a.inverse(), GF257::one());
  assert_eq!(&a + &a, GF257::zero());
  assert!(x_rand.get_ref_value().bits() <= (GF367::get_m() as u64));
  assert_eq!(GF367::from(b.get_prime_poly()), GF367::zero());
  Ok(())
}
