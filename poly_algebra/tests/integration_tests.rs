#[cfg(test)]
mod tests
{
  use num_bigint::{BigUint, ParseBigIntError};
  use num_traits::{Num, One, Zero};
  use poly_algebra::gf::{GFArithmetic, GFDisplay, GFGetters, GF163, GF167, GF173, GF179, GF191, GF233, GF257, GF307, GF367, GF431};
  use poly_algebra::gf_arithmetic::_inverse_old;
  use poly_algebra::helpers::generate_num;
  use proptest::arbitrary::any;
  use proptest::collection::vec;
  use proptest::prelude::Strategy;
  use proptest::proptest;
  use rand_chacha::ChaCha20Rng;
  use rand_core::{CryptoRngCore, SeedableRng};

  const ITERATIONS_NUM : usize = 200;
  const PROP_TEST_BIGUINT_BYTE_LEN : usize = 128;
  fn test_generated_len(rng : &mut impl CryptoRngCore, len : u64)
  {
    for _ in 0 .. ITERATIONS_NUM
    {
      let num = generate_num(rng, len);
      assert!(num.bits() <= len)
    }
  }

  fn arb_biguint() -> impl Strategy<Value = BigUint>
  {
    vec(any::<u8>(), 0 .. PROP_TEST_BIGUINT_BYTE_LEN)
      .prop_map(|bytes| BigUint::from_bytes_le(&bytes))
      .prop_filter("BigUint should not be zero", |n| *n != BigUint::zero())
  }
  fn arb_gf163() -> impl Strategy<Value = GF163> { arb_biguint().prop_map(|x| GF163::from(x)) }
  fn arb_gf167() -> impl Strategy<Value = GF167> { arb_biguint().prop_map(|x| GF167::from(x)) }
  fn arb_gf173() -> impl Strategy<Value = GF173> { arb_biguint().prop_map(|x| GF173::from(x)) }
  fn arb_gf179() -> impl Strategy<Value = GF179> { arb_biguint().prop_map(|x| GF179::from(x)) }
  fn arb_gf191() -> impl Strategy<Value = GF191> { arb_biguint().prop_map(|x| GF191::from(x)) }
  fn arb_gf233() -> impl Strategy<Value = GF233> { arb_biguint().prop_map(|x| GF233::from(x)) }
  fn arb_gf257() -> impl Strategy<Value = GF257> { arb_biguint().prop_map(|x| GF257::from(x)) }
  fn arb_gf307() -> impl Strategy<Value = GF307> { arb_biguint().prop_map(|x| GF307::from(x)) }
  fn arb_gf367() -> impl Strategy<Value = GF367> { arb_biguint().prop_map(|x| GF367::from(x)) }
  fn arb_gf431() -> impl Strategy<Value = GF431> { arb_biguint().prop_map(|x| GF431::from(x)) }
  fn arb_two_gf163() -> impl Strategy<Value = (GF163, GF163)> { (arb_gf163(), arb_gf163()) }
  fn arb_two_gf167() -> impl Strategy<Value = (GF167, GF167)> { (arb_gf167(), arb_gf167()) }
  fn arb_two_gf173() -> impl Strategy<Value = (GF173, GF173)> { (arb_gf173(), arb_gf173()) }
  fn arb_two_gf179() -> impl Strategy<Value = (GF179, GF179)> { (arb_gf179(), arb_gf179()) }
  fn arb_two_gf191() -> impl Strategy<Value = (GF191, GF191)> { (arb_gf191(), arb_gf191()) }
  fn arb_two_gf233() -> impl Strategy<Value = (GF233, GF233)> { (arb_gf233(), arb_gf233()) }
  fn arb_two_gf257() -> impl Strategy<Value = (GF257, GF257)> { (arb_gf257(), arb_gf257()) }
  fn arb_two_gf307() -> impl Strategy<Value = (GF307, GF307)> { (arb_gf307(), arb_gf307()) }
  fn arb_two_gf367() -> impl Strategy<Value = (GF367, GF367)> { (arb_gf367(), arb_gf367()) }
  fn arb_two_gf431() -> impl Strategy<Value = (GF431, GF431)> { (arb_gf431(), arb_gf431()) }
  fn arb_three_gf163() -> impl Strategy<Value = (GF163, GF163, GF163)> { (arb_gf163(), arb_gf163(), arb_gf163()) }
  fn arb_three_gf167() -> impl Strategy<Value = (GF167, GF167, GF167)> { (arb_gf167(), arb_gf167(), arb_gf167()) }
  fn arb_three_gf173() -> impl Strategy<Value = (GF173, GF173, GF173)> { (arb_gf173(), arb_gf173(), arb_gf173()) }
  fn arb_three_gf179() -> impl Strategy<Value = (GF179, GF179, GF179)> { (arb_gf179(), arb_gf179(), arb_gf179()) }
  fn arb_three_gf191() -> impl Strategy<Value = (GF191, GF191, GF191)> { (arb_gf191(), arb_gf191(), arb_gf191()) }
  fn arb_three_gf233() -> impl Strategy<Value = (GF233, GF233, GF233)> { (arb_gf233(), arb_gf233(), arb_gf233()) }
  fn arb_three_gf257() -> impl Strategy<Value = (GF257, GF257, GF257)> { (arb_gf257(), arb_gf257(), arb_gf257()) }
  fn arb_three_gf307() -> impl Strategy<Value = (GF307, GF307, GF307)> { (arb_gf307(), arb_gf307(), arb_gf307()) }
  fn arb_three_gf367() -> impl Strategy<Value = (GF367, GF367, GF367)> { (arb_gf367(), arb_gf367(), arb_gf367()) }
  fn arb_three_gf431() -> impl Strategy<Value = (GF431, GF431, GF431)> { (arb_gf431(), arb_gf431(), arb_gf431()) }

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
  #[test]
  fn prime_poly_test()
  {
    assert_eq!(
      GF163::zero().get_prime_poly(),
      BigUint::from_str_radix("800000000000000000000000000000000000000c9", 16).unwrap()
    );
    assert_eq!(
      GF167::zero().get_prime_poly(),
      BigUint::from_str_radix("800000000000000000000000000000000000000041", 16).unwrap()
    );
    assert_eq!(
      GF173::zero().get_prime_poly(),
      BigUint::from_str_radix("20000000000000000000000000000000000000000407", 16).unwrap()
    );
    assert_eq!(
      GF179::zero().get_prime_poly(),
      BigUint::from_str_radix("800000000000000000000000000000000000000000017", 16).unwrap()
    );
    assert_eq!(
      GF191::zero().get_prime_poly(),
      BigUint::from_str_radix("800000000000000000000000000000000000000000000201", 16).unwrap()
    );
    assert_eq!(
      GF233::zero().get_prime_poly(),
      BigUint::from_str_radix("20000000000000000000000000000000000000000000000000000000213", 16).unwrap()
    );
    assert_eq!(
      GF257::zero().get_prime_poly(),
      BigUint::from_str_radix("20000000000000000000000000000000000000000000000000000000000001001", 16).unwrap()
    );
    assert_eq!(
      GF307::zero().get_prime_poly(),
      BigUint::from_str_radix(
        "80000000000000000000000000000000000000000000000000000000000000000000000000115",
        16
      )
      .unwrap()
    );
    assert_eq!(
      GF367::zero().get_prime_poly(),
      BigUint::from_str_radix(
        "80000000000000000000000000000000000000000000000000000000000000000000000000000000000000200001",
        16
      )
      .unwrap()
    );
    assert_eq!(
      GF431::zero().get_prime_poly(),
      BigUint::from_str_radix(
        "80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002b",
        16
      )
      .unwrap()
    );
  }

  // Associativity
  proptest! {
      #[test]
      fn associativity_test_163((a,b,c) in arb_three_gf163()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_167((a,b,c) in arb_three_gf167()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_173((a,b,c) in arb_three_gf173()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_179((a,b,c) in arb_three_gf179()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_191((a,b,c) in arb_three_gf191()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_233((a,b,c) in arb_three_gf233()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_257((a,b,c) in arb_three_gf257()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_307((a,b,c) in arb_three_gf307()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_367((a,b,c) in arb_three_gf367()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }
  proptest! {
      #[test]
      fn associativity_test_431((a,b,c) in arb_three_gf431()) {
        assert_eq!(c.clone() * (&b + &a), &c * &b + &a * &c)
      }
  }

  // Square $(a+b)^2 = a^2 + b^2$
  proptest! {
      #[test]
      fn square_test_163((a,b) in arb_two_gf163()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_167((a,b) in arb_two_gf167()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_173((a,b) in arb_two_gf173()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_179((a,b) in arb_two_gf179()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_191((a,b) in arb_two_gf191()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_233((a,b) in arb_two_gf233()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_257((a,b) in arb_two_gf257()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_307((a,b) in arb_two_gf307()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_367((a,b) in arb_two_gf367()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }
  proptest! {
      #[test]
      fn square_test_431((a,b) in arb_two_gf431()) {
        assert_eq!(a.square() + b.square(), (a + b).square())
      }
  }

  // Powering
  // $a^{2^{m} - 1} = 1$
  // $a^{2^{m}} = a$
  fn calc_pow(m : u32) -> BigUint { (BigUint::one() << m) - BigUint::one() }

  proptest! {
      #[test]
      fn pow_test_163(a in arb_gf163()) {
        assert_eq!(a.pow(calc_pow(GF163::get_m())), GF163::one());
        assert_eq!(a.pow(calc_pow(GF163::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_167(a in arb_gf167()) {
        assert_eq!(a.pow(calc_pow(GF167::get_m())), GF167::one());
        assert_eq!(a.pow(calc_pow(GF167::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_173(a in arb_gf173()) {
        assert_eq!(a.pow(calc_pow(GF173::get_m())), GF173::one());
        assert_eq!(a.pow(calc_pow(GF173::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_179(a in arb_gf179()) {
        assert_eq!(a.pow(calc_pow(GF179::get_m())), GF179::one());
        assert_eq!(a.pow(calc_pow(GF179::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_191(a in arb_gf191()) {
        assert_eq!(a.pow(calc_pow(GF191::get_m())), GF191::one());
        assert_eq!(a.pow(calc_pow(GF191::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_233(a in arb_gf233()) {
        assert_eq!(a.pow(calc_pow(GF233::get_m())), GF233::one());
        assert_eq!(a.pow(calc_pow(GF233::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_257(a in arb_gf257()) {
        assert_eq!(a.pow(calc_pow(GF257::get_m())), GF257::one());
        assert_eq!(a.pow(calc_pow(GF257::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_307(a in arb_gf307()) {
        assert_eq!(a.pow(calc_pow(GF307::get_m())), GF307::one());
        assert_eq!(a.pow(calc_pow(GF307::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_367(a in arb_gf367()) {
        assert_eq!(a.pow(calc_pow(GF367::get_m())), GF367::one());
        assert_eq!(a.pow(calc_pow(GF367::get_m()) + BigUint::one()), a);
      }
  }
  proptest! {
      #[test]
      fn pow_test_431(a in arb_gf431()) {
        assert_eq!(a.pow(calc_pow(GF431::get_m())), GF431::one());
        assert_eq!(a.pow(calc_pow(GF431::get_m()) + BigUint::one()), a);
      }
  }

  // Inverse
  proptest! {
      #[test]
      fn inverse_test_163(a in arb_gf163()) {
        assert_eq!(a.inverse() * a, GF163::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_167(a in arb_gf167()) {
        assert_eq!(a.inverse() * a, GF167::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_173(a in arb_gf173()) {
        assert_eq!(a.inverse() * a, GF173::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_179(a in arb_gf179()) {
        assert_eq!(a.inverse() * a, GF179::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_191(a in arb_gf191()) {
        assert_eq!(a.inverse() * a, GF191::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_233(a in arb_gf233()) {
        assert_eq!(a.inverse() * a, GF233::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_257(a in arb_gf257()) {
        assert_eq!(a.inverse() * a, GF257::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_307(a in arb_gf307()) {
        assert_eq!(a.inverse() * a, GF307::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_367(a in arb_gf367()) {
        assert_eq!(a.inverse() * a, GF367::one());
      }
  }
  proptest! {
      #[test]
      fn inverse_test_431(a in arb_gf431()) {
        assert_eq!(a.inverse() * a, GF431::one());
      }
  }

  // Addition

  proptest! {
      #[test]
      fn add_test_163(a in arb_gf163()) {
        assert_eq!(a.clone() + a, GF163::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_167(a in arb_gf167()) {
        assert_eq!(a.clone() + a, GF167::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_173(a in arb_gf173()) {
        assert_eq!(a.clone() + a, GF173::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_179(a in arb_gf179()) {
        assert_eq!(a.clone() + a, GF179::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_191(a in arb_gf191()) {
        assert_eq!(a.clone() + a, GF191::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_233(a in arb_gf233()) {
        assert_eq!(a.clone() + a, GF233::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_257(a in arb_gf257()) {
        assert_eq!(a.clone() + a, GF257::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_307(a in arb_gf307()) {
        assert_eq!(a.clone() + a, GF307::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_367(a in arb_gf367()) {
        assert_eq!(a.clone() + a, GF367::zero());
      }
  }
  proptest! {
      #[test]
      fn add_test_431(a in arb_gf431()) {
        assert_eq!(a.clone() + a, GF431::zero());
      }
  }

  // Trace
  proptest! {
      #[test]
      fn trace_test_163((a,b) in arb_two_gf163()) {
        assert_eq!(GF163::zero().trace(), BigUint::zero());
        assert_eq!(GF163::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace());
      }
  }
  proptest! {
      #[test]
      fn trace_test_167((a,b) in arb_two_gf167()) {
        assert_eq!(GF167::zero().trace(), BigUint::zero());
        assert_eq!(GF167::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace());
      }
  }
  proptest! {
      #[test]
      fn trace_test_173((a,b) in arb_two_gf173()) {
        assert_eq!(GF173::zero().trace(), BigUint::zero());
        assert_eq!(GF173::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace());
      }
  }
  proptest! {
      #[test]
      fn trace_test_179((a,b) in arb_two_gf179()) {
        assert_eq!(GF179::zero().trace(), BigUint::zero());
        assert_eq!(GF179::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }
  proptest! {
      #[test]
      fn trace_test_191((a,b) in arb_two_gf191()) {
        assert_eq!(GF191::zero().trace(), BigUint::zero());
        assert_eq!(GF191::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }
  proptest! {
      #[test]
      fn trace_test_233((a,b) in arb_two_gf233()) {
        assert_eq!(GF233::zero().trace(), BigUint::zero());
        assert_eq!(GF233::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }
  proptest! {
      #[test]
      fn trace_test_257((a,b) in arb_two_gf257()) {
        assert_eq!(GF257::zero().trace(), BigUint::zero());
        assert_eq!(GF257::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }
  proptest! {
      #[test]
      fn trace_test_307((a,b) in arb_two_gf307()) {
        assert_eq!(GF307::zero().trace(), BigUint::zero());
        assert_eq!(GF307::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }
  proptest! {
      #[test]
      fn trace_test_367((a,b) in arb_two_gf367()) {
        assert_eq!(GF367::zero().trace(), BigUint::zero());
        assert_eq!(GF367::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }
  proptest! {
      #[test]
      fn trace_test_431((a,b) in arb_two_gf431()) {
        assert_eq!(GF431::zero().trace(), BigUint::zero());
        assert_eq!(GF431::one().trace(), BigUint::one());
        assert_eq!((a.clone() + &b).trace(), a.trace() ^ b.trace());
        assert_eq!(a.square().trace(), a.trace())
      }
  }

  // HTrace
  proptest! {
      #[test]
      fn htrace_test_163((a,b) in arb_two_gf163()) {
        assert_eq!(GF163::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_167((a,b) in arb_two_gf167()) {
        assert_eq!(GF167::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_173((a,b) in arb_two_gf173()) {
        assert_eq!(GF173::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_179((a,b) in arb_two_gf179()) {
        assert_eq!(GF179::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_191((a,b) in arb_two_gf191()) {
        assert_eq!(GF191::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_233((a,b) in arb_two_gf233()) {
        assert_eq!(GF233::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_257((a,b) in arb_two_gf257()) {
        assert_eq!(GF257::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_307((a,b) in arb_two_gf307()) {
        assert_eq!(GF307::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_367((a,b) in arb_two_gf367()) {
        assert_eq!(GF367::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
  proptest! {
      #[test]
      fn htrace_test_431((a,b) in arb_two_gf431()) {
        assert_eq!(GF431::zero().htrace(), BigUint::zero());
        assert_eq!((a.clone() + &b).htrace(), a.htrace() ^ b.htrace());
      }
  }
}
