use poly_algebra::helpers::generate_num;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use rust_ec::affine_point::AffinePoint;
use rust_ec::binary_ec::BinaryEC;
use rust_ec::helpers::generate_random_affine_point;

fn main()
{
  let mut rng = ChaCha20Rng::from_entropy();
  let ec = BinaryEC::generate_m257_pb_curve();
  let q = generate_random_affine_point(&mut rng, &ec);
  let n_p = {
    let p = ec.get_bp();
    let n = generate_num(&mut rng, ec.get_ref_ord().bits() - 1);
    ec.mul(&p, n)
  };
  let n_p_q = ec.add(&q, &n_p);
  let n_p_q_inv = n_p_q.negative();
  let q_inv = q.negative();
  let n_p_inv = n_p.negative();
  assert_eq!(ec.add(&n_p_q, &n_p_q_inv), AffinePoint::Infinity);
  assert_eq!(ec.add(&n_p, &n_p_inv), AffinePoint::Infinity);
  assert_eq!(ec.add(&q, &q_inv), AffinePoint::Infinity);
  assert!(ec.check_affine_point(&q));
  assert!(ec.check_affine_point(&n_p));
  assert!(ec.check_affine_point(&n_p_q));
  assert!(ec.check_affine_point(&n_p_q_inv));
  assert!(ec.check_affine_point(&q_inv));
  assert!(ec.check_affine_point(&n_p_inv));

  let n_p_packed = n_p.pack();
  let n_p_unpacked = ec.unpack_affine_point(&n_p_packed);
  assert_eq!(n_p_unpacked, n_p);
}
