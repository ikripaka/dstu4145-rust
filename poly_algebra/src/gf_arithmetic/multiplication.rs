use std::mem;
use std::ops::{Shl, Shr};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::gf_arithmetic::addition::add;
use crate::gf_arithmetic::reduction::module_reduction;
use crate::helpers::to_binary_be;

//todo: implement Montgomery multiplication
pub fn mul(a: &mut BigUint, b: &BigUint, prime_poly: &BigUint) {
  let mut c = BigUint::zero();
  let b_bit_len = b.bits();
  let mut a_copy = a.clone();
  let mut b_copy = b.clone();
  let one = BigUint::one();
  for _ in 0..b_bit_len {
    if &b_copy & &one == one {
      add(&mut c, &a_copy);
    }
    a_copy <<= 1;
    b_copy >>= 1;
  }
  let _ = mem::replace(a, c);
  module_reduction(a, prime_poly);
}

pub fn square(a: &mut BigUint, prime_poly: &BigUint) {
  let mut res = BigUint::zero();
  let mut tmp = a.clone();
  let one = BigUint::one();
  for i in 0..a.bits() {
    if &tmp & &one == one {
      // i << 1 = 2*i
      add(&mut res, &(&one << (i << 1)))
    }
    tmp >>= 1;
  }
  let _ = mem::replace(a, res);
  module_reduction(a, prime_poly);
}

pub fn pow(a: &mut BigUint, prime_poly: &BigUint, power: &BigUint) {
  let mut c = BigUint::one();

  let mut window = Vec::with_capacity(16);
  window.push(BigUint::one());
  window.push(a.clone());
  for i in 2..16 {
    let mut a_cloned = a.clone();
    mul(&mut a_cloned, &window[i - 1], prime_poly);
    window.push(a_cloned);
  }
  let power_hex_list = power.to_radix_le(16);
  for (j, h) in power_hex_list.iter().enumerate().rev() {
    mul(&mut c, &window[*h as usize], prime_poly);
    if j != 0 {
      for _ in 0..4 {
        square(&mut c, prime_poly)
      }
    }
  }
  let _ = mem::replace(a, c);
}

/// Reverse by multiplication
/// todo: maybe implement euclid algorithm
pub fn inverse(a: &BigUint, prime_poly: &BigUint) -> BigUint {
  let poly_size = prime_poly.bits();
  let two = BigUint::from(2_u8);
  let power = (&BigUint::one() << poly_size - 1) - two;
  println!("{}", power);
  let mut inverse = a.clone();
  pow(&mut inverse, prime_poly, &power);
  inverse
}
