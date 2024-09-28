#[cfg(test)]
mod tests
{
  use num_bigint::BigUint;
  use num_traits::{Num, One, Zero};
  use rand_chacha::ChaCha20Rng;
  use rand_core::{CryptoRngCore, SeedableRng};
  use poly_algebra::gf::gf_def::{GFArithmetic, GFDisplay, GFGetters};
  use poly_algebra::gf::gf_def::{GF239, GF3, GF307};
  use poly_algebra::helpers::{generate_num, to_binary_be};

  const ITERATIONS_NUM : usize = 200;
  fn test_generated_len(rng : &mut impl CryptoRngCore, len : u64)
  {
    for _ in 0 .. ITERATIONS_NUM
    {
      let num = generate_num(rng, len);
      assert!(num.bits() <= len)
    }
  }
  #[test]
  fn test_random_generation()
  {
    let mut rng = ChaCha20Rng::from_seed(Default::default());
    let mut len = 1;
    test_generated_len(&mut rng, len);
    len = 10;
    test_generated_len(&mut rng, len);
    len = 31;
    test_generated_len(&mut rng, len);
    len = 32;
    test_generated_len(&mut rng, len);
    len = 33;
    test_generated_len(&mut rng, len);
    len = 163;
    test_generated_len(&mut rng, len);
    len = 167;
    test_generated_len(&mut rng, len);
    len = 173;
    test_generated_len(&mut rng, len);
    len = 179;
    test_generated_len(&mut rng, len);
    len = 233;
    test_generated_len(&mut rng, len);
    len = 307;
    test_generated_len(&mut rng, len);
    len = 367;
    test_generated_len(&mut rng, len);
    len = 431;
    test_generated_len(&mut rng, len);
  }

  // todo: generate with macros test implementation of GF4
  #[test]
  fn prime_poly_test()
  {
    // assert_eq!(GF163::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF167::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF173::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF179::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF191::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF233::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    assert_eq!(
      GF239::zero().get_prime_poly(),
      BigUint::from_str_radix("800000000000000000000000000000000000000000000000000000008007", 16).unwrap()
    );
    // assert_eq!(GF257::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF307::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF367::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // assert_eq!(GF431::zero().get_prime_poly(), BigUint::from_str_radix("" ,16).unwrap());
    // println!("{:?}",to_upper_hex_be(&GF239::zero().get_prime_poly()));
  }

  #[test]
  fn multiplication_test()
  {
    // let mut x : Box<dyn poly_algebra::gf::gf_def::GFArithmeticObjSafe> = Box::new(GF3::one());
    // x = Box::new(GF3::one());
    {
      let a = GF239::from(
        BigUint::from_str_radix(
          "42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA",
          16,
        )
        .unwrap(),
      );
      let b = GF239::from(
        BigUint::from_str_radix(
          "CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA56D70E1C9BA62D2CB4FFF",
          16,
        )
        .unwrap(),
      );
      assert_eq!(&b * &a, &a * &b)
    }
    let mut a = GF3::from(BigUint::from_str_radix("2", 16).unwrap());
    let b = GF3::from(BigUint::from_str_radix("2", 16).unwrap());
    println!(
      "num: {}, a^5: {}, a*a: {}, trace: {}, htrace: {}, pow^7: {}",
      a.to_binary_be(),
      (&a.square() * &a.square() * &a).to_binary_be(),
      (a.square()).to_binary_be(),
      a.trace(),
      to_binary_be(&a.htrace()),
      a.pow(123_u128).to_binary_be()
    );
    for _ in 0 .. 15
    {
      a = &a * &b;
      println!(
        "num: {}, a^5: {}, a*a: {}, trace: {}, htrace: {}, pow^7: {}, inverse: {}, inverse_check: {}",
        a.to_binary_be(),
        (&a.square() * &a.square() * &a).to_binary_be(),
        (a.square()).to_binary_be(),
        a.trace(),
        to_binary_be(&a.htrace()),
        a.pow(1000000_u128).to_binary_be(),
        a.inverse().to_binary_be(),
        (&a * &a.inverse()).to_binary_be()
      )
    }

    // {
    //   let a = GF239::from(BigUint::from_str_radix("42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA", 16).unwrap());
    //   let b = GF239::from(BigUint::from_str_radix("CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA56D70E1C9BA62D2CB4FFF", 16).unwrap());
    //   let c = GF239::from(BigUint::from_str_radix("2D2CB43707C35204EF3C67CA56D70E1C9BA62D2CB4FFFCB43707C35204EF3C67CA42A7D756D70E1C9BA6", 16).unwrap());
    //   assert_eq!( &c * &(&b * &a), (&c* &a) * (&c * &b))
    // }
  }
  #[test]
  fn associativity_test()
  {
    {
      let a = GF239::from(BigUint::from(0x00000000FF112233_u128));
      let b = GF239::from(BigUint::from(0xAABBCCDD00000000_u128));
      let c = GF239::from(BigUint::from(0xAABBCCDD00000000AABBCCDD00000000_u128));
      println!("{:X}", (&a + &b));
      assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
    }
  }

  #[test]
  fn trace_test() {}

  #[test]
  fn htrace_test() {}

  #[test]
  fn inverse_test()
  {
    let a = GF239::from(
      BigUint::from_str_radix(
        "42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA",
        16,
      )
      .unwrap(),
    );
    let inv = a.inverse();
    let r = a * inv;
    println!("{r:X}")
  }

  #[test]
  fn test1()
  {
    {
      let a = GF239::from(
        BigUint::from_str_radix(
          "42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA",
          16,
        )
        .unwrap(),
      );
      let b = GF239::from(
        BigUint::from_str_radix(
          "CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA56D70E1C9BA62D2CB4FFF",
          16,
        )
        .unwrap(),
      );
      let x = &b * &a;
      // assert_eq!( &b * &a, &a * &b)
    }
  }
}
