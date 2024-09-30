#[cfg(test)]
mod tests
{
  use std::cmp::min;
  use num_bigint::BigUint;
  use num_traits::{Num, One};
  use poly_algebra::gf::{GF163, GF167, GF173, GF179, GF191, GF233, GF257, GF307, GF367, GF431};
  use rust_ec::affine_point::AffinePoint;
  use rust_ec::binary_ec::BinaryEC;

  use rand_chacha::ChaCha20Rng;
  use rand_core::{CryptoRngCore, SeedableRng};
  use poly_algebra::helpers::generate_num;
  use rust_ec::helpers::generate_random_affine_point;

  #[test]
  fn negative()
  {
    let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
    let mut a = AffinePoint::Point {
      x : GF163::from(BigUint::from_str_radix("2E2F85F5DD74CE983A5C4237229DAF8A3F35823BE", 16).unwrap()),
      y : GF163::from(BigUint::from_str_radix("3826F008A8C51D7B95284D9D03FF0E00CE2CD723A", 16).unwrap()),
    };
    let b = AffinePoint::Point {
      x : GF163::from(BigUint::from_str_radix("2E2F85F5DD74CE983A5C4237229DAF8A3F35823BE", 16).unwrap()),
      y : GF163::from(BigUint::from_str_radix("3826F008A8C51D7B95284D9D03FF0E00CE2CD723A", 16).unwrap()),
    };

    println!("i: 0, a: {:X?}, b: {:X?}", a, b);
    for i in 0 .. 20
    {
      a = a.add(&ec, &b);
      println!("i: {i}, a: {:X?}", a);
    }
    assert_eq!(a, b.mul(&ec, 21_u8));
    println!("a: {:X?}", a.add(&ec, &a.negative()));
    println!(
      "a: {:X?}",
      a.mul(
        &ec,
        BigUint::from_str_radix("400000000000000000002BEC12BE2262D39BCF14D", 16).unwrap()
      )
    );
  }

  #[test]
  fn point_generation_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let p = generate_random_affine_point(&mut rng, &ec);
      assert!(ec.check_affine_point(p))
    }
  }

  #[test]
  fn point_add_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
  }

  #[test]
  fn point_add_neutral_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = AffinePoint::Infinity;
      assert_eq!(ec.add(&a, &b), ec.add(&b, &a))
    }
  }

  #[test]
  fn inverse_add_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      let b = a.negative();
      let a_b = ec.add(&a, &b);
      let b_a = ec.add(&b, &a);
      assert_eq!(a_b, AffinePoint::Infinity);
      assert_eq!(b_a, AffinePoint::Infinity);
      assert_eq!(a_b, b_a);
    }
  }

  #[test]
  fn point_doubling_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let a = generate_random_affine_point(&mut rng, &ec);
      assert_eq!(ec.double(&a), ec.mul(&a, 2_u8))
    }
  }

  /// (-k)P = k(-P)
  #[test]
  fn point_negative_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits()) % ec.get_ref_ord();
      let minus_k = (ec.get_ref_ord() - &k) % ec.get_ref_ord();
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let minus_k = ec.get_ref_ord() - &k;
      let p = ec.get_bp();
      let p_negative = p.negative();
      assert_eq!(ec.mul(&p, minus_k), ec.mul(&p_negative, k))
    }
  }

  #[test]
  fn point_mul_ord_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
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

  #[test]
  fn point_packing_test()
  {
    let mut rng = ChaCha20Rng::from_entropy();
    {
      let ec = BinaryEC::<GF163>::generate_m163_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF167>::generate_m167_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF173>::generate_m173_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF179>::generate_m179_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF191>::generate_m191_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF233>::generate_m233_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF257>::generate_m257_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF307>::generate_m307_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF367>::generate_m367_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
    {
      let ec = BinaryEC::<GF431>::generate_m431_pb_curve();
      let k = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
      let q = ec.mul(ec.get_ref_bp(), k);
      let packed_point = q.pack();
      let unpacked_point = AffinePoint::unpack(&packed_point, &ec);
      assert_eq!(q, unpacked_point);
    }
  }
}
