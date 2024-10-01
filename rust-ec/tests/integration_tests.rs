#[cfg(test)]
mod tests
{
  use num_bigint::BigUint;
  use num_traits::Zero;
  use proptest::arbitrary::any;
  use proptest::collection::vec;
  use proptest::prelude::Strategy;
  use proptest::proptest;
  use poly_algebra::gf::{GFArithmetic, GF163, GF167, GF173, GF179, GF191, GF233, GF257, GF307, GF367, GF431};
  use rust_ec::affine_point::AffinePoint;
  use rust_ec::binary_ec::BinaryEC;

  use rand_chacha::ChaCha20Rng;
  use rand_core::{CryptoRngCore, SeedableRng};
  use poly_algebra::helpers::{generate_num, solve_quadratic_equation_in_field};
  use rust_ec::helpers::generate_random_affine_point;

  const PROP_TEST_BIGUINT_BYTE_LEN : usize = 128;
  pub fn generate_affine_point_local<'a, T : GFArithmetic<'a>>(u : T, ec : &BinaryEC<T>) -> Option<AffinePoint<T>>
  {
    let w = {
      let u_2 = u.square();
      let u_3 = u.clone() * u_2.clone();
      let a = ec.get_ref_a().as_field_el();
      u_3 + a * u_2 + ec.get_b()
    };
    solve_quadratic_equation_in_field::<T>(&u, &w).map(|(y, _)| AffinePoint::Point { x : u, y })
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

  fn arb_affine_point_gf163() -> impl Strategy<Value = AffinePoint<GF163>>
  {
    let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
    arb_gf163().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf167() -> impl Strategy<Value = AffinePoint<GF167>>
  {
    let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
    arb_gf167().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf173() -> impl Strategy<Value = AffinePoint<GF173>>
  {
    let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
    arb_gf173().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf179() -> impl Strategy<Value = AffinePoint<GF179>>
  {
    let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
    arb_gf179().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf191() -> impl Strategy<Value = AffinePoint<GF191>>
  {
    let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
    arb_gf191().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf233() -> impl Strategy<Value = AffinePoint<GF233>>
  {
    let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
    arb_gf233().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf257() -> impl Strategy<Value = AffinePoint<GF257>>
  {
    let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
    arb_gf257().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf307() -> impl Strategy<Value = AffinePoint<GF307>>
  {
    let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
    arb_gf307().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf367() -> impl Strategy<Value = AffinePoint<GF367>>
  {
    let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
    arb_gf367().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_affine_point_gf431() -> impl Strategy<Value = AffinePoint<GF431>>
  {
    let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
    arb_gf431().prop_filter_map("Unable to find y coordinate for given x", move |x| {
      generate_affine_point_local(x, &ec)
    })
  }
  fn arb_two_affine_points_gf163() -> impl Strategy<Value = (AffinePoint<GF163>, AffinePoint<GF163>)>
  {
    (arb_affine_point_gf163(), arb_affine_point_gf163())
  }
  fn arb_two_affine_points_gf167() -> impl Strategy<Value = (AffinePoint<GF167>, AffinePoint<GF167>)>
  {
    (arb_affine_point_gf167(), arb_affine_point_gf167())
  }
  fn arb_two_affine_points_gf173() -> impl Strategy<Value = (AffinePoint<GF173>, AffinePoint<GF173>)>
  {
    (arb_affine_point_gf173(), arb_affine_point_gf173())
  }
  fn arb_two_affine_points_gf179() -> impl Strategy<Value = (AffinePoint<GF179>, AffinePoint<GF179>)>
  {
    (arb_affine_point_gf179(), arb_affine_point_gf179())
  }
  fn arb_two_affine_points_gf191() -> impl Strategy<Value = (AffinePoint<GF191>, AffinePoint<GF191>)>
  {
    (arb_affine_point_gf191(), arb_affine_point_gf191())
  }
  fn arb_two_affine_points_gf233() -> impl Strategy<Value = (AffinePoint<GF233>, AffinePoint<GF233>)>
  {
    (arb_affine_point_gf233(), arb_affine_point_gf233())
  }
  fn arb_two_affine_points_gf257() -> impl Strategy<Value = (AffinePoint<GF257>, AffinePoint<GF257>)>
  {
    (arb_affine_point_gf257(), arb_affine_point_gf257())
  }
  fn arb_two_affine_points_gf307() -> impl Strategy<Value = (AffinePoint<GF307>, AffinePoint<GF307>)>
  {
    (arb_affine_point_gf307(), arb_affine_point_gf307())
  }
  fn arb_two_affine_points_gf367() -> impl Strategy<Value = (AffinePoint<GF367>, AffinePoint<GF367>)>
  {
    (arb_affine_point_gf367(), arb_affine_point_gf367())
  }
  fn arb_two_affine_points_gf431() -> impl Strategy<Value = (AffinePoint<GF431>, AffinePoint<GF431>)>
  {
    (arb_affine_point_gf431(), arb_affine_point_gf431())
  }
  fn arb_random_ec_num_mod_gf163() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF163>::generate_m163_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf167() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF167>::generate_m167_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf173() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF173>::generate_m173_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf179() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF179>::generate_m179_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf191() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF191>::generate_m191_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf233() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF233>::generate_m233_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf257() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF257>::generate_m257_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf307() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF307>::generate_m307_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf367() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF367>::generate_m367_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }
  fn arb_random_ec_num_mod_gf431() -> impl Strategy<Value = BigUint>
  {
    let n = BinaryEC::<GF431>::generate_m431_pb_curve().get_ord();
    arb_biguint().prop_map(move |x| x % &n)
  }

  // Add negative
  proptest! {
      #[test]
      fn negative_test_163(a in arb_affine_point_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_167(a in arb_affine_point_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_173(a in arb_affine_point_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        assert_eq!(ec.add(&a,&a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_179(a in arb_affine_point_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
         assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_191(a in arb_affine_point_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_233(a in arb_affine_point_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
         assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_257(a in arb_affine_point_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_307(a in arb_affine_point_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_367(a in arb_affine_point_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }
  proptest! {
      #[test]
      fn negative_test_431(a in arb_affine_point_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        assert_eq!(ec.add(&a, &a.negative()), AffinePoint::Infinity)
      }
  }

  // Addition

  proptest! {
      #[test]
      fn add_test_163((a,b) in arb_two_affine_points_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_167((a,b) in arb_two_affine_points_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_173((a,b) in arb_two_affine_points_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_179((a,b) in arb_two_affine_points_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_191((a,b) in arb_two_affine_points_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_233((a,b) in arb_two_affine_points_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
         assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_257((a,b) in arb_two_affine_points_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_307((a,b) in arb_two_affine_points_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_367((a,b) in arb_two_affine_points_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_test_431((a,b) in arb_two_affine_points_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }

  // Add neutral

  proptest! {
      #[test]
      fn add_negative_test_163(a in arb_affine_point_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_167(a in arb_affine_point_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_173(a in arb_affine_point_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn nadd_negativetest_179(a in arb_affine_point_gf179()) {
        let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_191(a in arb_affine_point_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn nadd_negativetest_233(a in arb_affine_point_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_257(a in arb_affine_point_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_307(a in arb_affine_point_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_367(a in arb_affine_point_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }
  proptest! {
      #[test]
      fn add_negative_test_431(a in arb_affine_point_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        let b = AffinePoint::Infinity;
        assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
      }
  }

  // Inverse
  proptest! {
      #[test]
      fn inverse_test_163(a in arb_affine_point_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_167(a in arb_affine_point_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_173(a in arb_affine_point_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_179(a in arb_affine_point_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
         let b = a.negative();
         let a_b = ec.add(&a, &b);
         let b_a = ec.add(&b, &a);
         assert_eq!(a_b, AffinePoint::Infinity);
         assert_eq!(b_a, AffinePoint::Infinity);
         assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_191(a in arb_affine_point_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_233(a in arb_affine_point_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
         let b = a.negative();
         let a_b = ec.add(&a, &b);
         let b_a = ec.add(&b, &a);
         assert_eq!(a_b, AffinePoint::Infinity);
         assert_eq!(b_a, AffinePoint::Infinity);
         assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_257(a in arb_affine_point_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_307(a in arb_affine_point_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_367(a in arb_affine_point_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }
  proptest! {
      #[test]
      fn inverse_test_431(a in arb_affine_point_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        let b = a.negative();
        let a_b = ec.add(&a, &b);
        let b_a = ec.add(&b, &a);
        assert_eq!(a_b, AffinePoint::Infinity);
        assert_eq!(b_a, AffinePoint::Infinity);
        assert_eq!(a_b, b_a);
      }
  }

  // Doubling
  proptest! {
      #[test]
      fn doubling_test_163(a in arb_affine_point_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_167(a in arb_affine_point_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_173(a in arb_affine_point_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_179(a in arb_affine_point_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
         assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_191(a in arb_affine_point_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_233(a in arb_affine_point_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
         assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_257(a in arb_affine_point_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_307(a in arb_affine_point_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_367(a in arb_affine_point_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }
  proptest! {
      #[test]
      fn doubling_test_431(a in arb_affine_point_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
      }
  }

  // (-k)P = k(-P)
  // Negative coefficient test
  proptest! {
      #[test]
      fn negative_coef_test_163(k in arb_random_ec_num_mod_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_167(k in arb_random_ec_num_mod_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_173(k in arb_random_ec_num_mod_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_179(k in arb_random_ec_num_mod_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_191(k in arb_random_ec_num_mod_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_233(k in arb_random_ec_num_mod_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_257(k in arb_random_ec_num_mod_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_307(k in arb_random_ec_num_mod_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_367(k in arb_random_ec_num_mod_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }
  proptest! {
      #[test]
      fn negative_coef_test_431(k in arb_random_ec_num_mod_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        let minus_k = ec.get_ref_ord() - &k;
        let p = ec.get_bp();
        let p_negative = p.negative();
        assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
      }
  }

  // Packing
  proptest! {
      #[test]
      fn packing_test_163(k in arb_random_ec_num_mod_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_167(k in arb_random_ec_num_mod_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_173(k in arb_random_ec_num_mod_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_179(k in arb_random_ec_num_mod_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
         let q = ec.mul(ec.get_ref_bp(), k);
         let packed_point = q.pack();
         let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
         assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_191(k in arb_random_ec_num_mod_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_233(k in arb_random_ec_num_mod_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
         let q = ec.mul(ec.get_ref_bp(), k);
         let packed_point = q.pack();
         let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
         assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_257(k in arb_random_ec_num_mod_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_307(k in arb_random_ec_num_mod_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_367(k in arb_random_ec_num_mod_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }
  proptest! {
      #[test]
      fn packing_test_431(k in arb_random_ec_num_mod_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        let q = ec.mul(ec.get_ref_bp(), k);
        let packed_point = q.pack();
        let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
        assert_eq!(q, unpacked_point);
      }
  }

  // Point generation

  proptest! {
      #[test]
      fn point_generation_test_163(p in arb_affine_point_gf163()) {
        let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_167(p in arb_affine_point_gf167()) {
        let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_173(p in arb_affine_point_gf173()) {
        let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_179(p in arb_affine_point_gf179()) {
         let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_191(p in arb_affine_point_gf191()) {
        let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_233(p in arb_affine_point_gf233()) {
         let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_257(p in arb_affine_point_gf257()) {
        let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_307(p in arb_affine_point_gf307()) {
        let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_367(p in arb_affine_point_gf367()) {
        let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }
  proptest! {
      #[test]
      fn point_generation_test_431(p in arb_affine_point_gf431()) {
        let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
        assert!(ec.check_affine_point(&p))
      }
  }

  #[test]
  fn bp_point_mul_ord_test()
  {
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let p = ec.get_bp();
      assert_eq!(ec.mul(&p, ec.get_ord()), AffinePoint::Infinity)
    }
  }
}
