use std::mem;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

/// Function that implements XORed addition of polynomials in binary field.
pub fn add(a : &mut BigUint, b : &BigUint) { *a ^= b; }

/// Function implements regular multiplication with linear shift.
pub fn mul(a : &mut BigUint, b : &BigUint, prime_poly : &BigUint)
{
  let mut c = BigUint::zero();
  let b_bit_len = b.bits();
  let mut a_copy = a.clone();
  let mut b_copy = b.clone();
  let one = BigUint::one();
  for _ in 0 .. b_bit_len
  {
    if &b_copy & &one == one
    {
      add(&mut c, &a_copy);
    }
    a_copy <<= 1;
    b_copy >>= 1;
  }
  let _ = mem::replace(a, c);
  module_reduction(a, prime_poly);
}

/// Function that calculates square of polynomial with help of thinning out with zeros.
pub fn square(a : &mut BigUint, prime_poly : &BigUint)
{
  let mut res = BigUint::zero();
  let mut tmp = a.clone();
  let one = BigUint::one();
  for i in 0 .. a.bits()
  {
    if &tmp & &one == one
    {
      // i << 1 = 2*i
      add(&mut res, &(&one << (i << 1)))
    }
    tmp >>= 1;
  }
  let _ = mem::replace(a, res);
  module_reduction(a, prime_poly);
}

/// Function that implements window method of powering numbers to some power.
pub fn pow(a : &mut BigUint, prime_poly : &BigUint, power : &BigUint)
{
  let mut c = BigUint::one();
  let mut window = Vec::with_capacity(16);
  window.push(BigUint::one());
  window.push(a.clone());
  for i in 2 .. 16
  {
    let mut a_cloned = a.clone();
    mul(&mut a_cloned, &window[i - 1], prime_poly);
    window.push(a_cloned);
  }
  let power_hex_list = power.to_radix_le(16);
  for (j, h) in power_hex_list.iter().enumerate().rev()
  {
    mul(&mut c, &window[*h as usize], prime_poly);
    if j != 0
    {
      for _ in 0 .. 4
      {
        square(&mut c, prime_poly)
      }
    }
  }
  let _ = mem::replace(a, c);
}

/// Function that finds reverse element in binary field for polynomial with usage of simple
/// powering number to the $2^{m} - 2$ power.
/// (because $a^{2^{m} - 2} * a = a^{2^{m} - 1} = 1$)
pub fn _inverse_old(a : &BigUint, prime_poly : &BigUint) -> BigUint
{
  let poly_size = prime_poly.bits();
  let two = BigUint::from(2_u8);
  let power = (&BigUint::one() << (poly_size - 1)) - two;
  let mut inverse = a.clone();
  pow(&mut inverse, prime_poly, &power);
  inverse
}

/// Function that implements regular polyn0mial reduction in binary field.
pub fn module_reduction(a : &mut BigUint, prime_poly : &BigUint)
{
  let mut a_bits = a.bits();
  let poly_size = prime_poly.bits();
  while a_bits >= poly_size
  {
    add(a, &(prime_poly << (a_bits - poly_size)));
    a_bits = a.bits();
  }
}

/// Function to calculate trace of the element in field which is implemented from `6.5` algorithm.
pub fn trace(a : &BigUint, prime_poly : &BigUint) -> BigUint
{
  let poly_size = prime_poly.bits();
  let mut t = a.clone();
  for _ in 1 .. poly_size - 1
  {
    square(&mut t, prime_poly);
    add(&mut t, a);
  }
  t
}
/// Function to calculate half trace which is implemented from algorithm `6.6`.
pub fn htrace(a : &BigUint, prime_poly : &BigUint) -> BigUint
{
  let poly_size = prime_poly.bits();
  // Half trace only accepts odd values
  assert_eq!(poly_size % 2, 0);
  let mut t = a.clone();
  for _ in 1 ..= ((poly_size - 1) >> 1)
  {
    square(&mut t, prime_poly);
    square(&mut t, prime_poly);
    add(&mut t, a);
  }
  t
}

pub fn inverse(el : &BigUint, prime_poly : &BigUint) -> BigUint
{
  let (mut r_0, mut r_1) = (el.clone(), prime_poly.clone());
  let (mut u_0, mut u_1) = (BigUint::one(), BigUint::zero());
  let (mut v_0, mut v_1) = (BigUint::zero(), BigUint::one());
  while !r_0.is_zero()
  {
    let (quotient, remainder) = divide(&r_1, &r_0);
    (r_1, r_0) = (r_0, remainder);

    if r_0.is_zero(){
      break
    }

    let u_next = {
      let mut tmp = u_1.clone();
      mul(&mut tmp, &quotient, prime_poly);
      &u_0 ^ tmp
    };
    let v_next = {
      let mut tmp = v_1.clone();
      mul(&mut tmp, &quotient, prime_poly);
      &v_0 ^ tmp
    };
    (u_0, u_1) = (u_1, u_next);
    (v_0, v_1) = (v_1, v_next);
  }
  v_1
}


/// Function divides polynomial into 2 parts (_Denominator_, _Residual_).
/// a = prime_poly * d + r
pub fn divide(a : &BigUint, prime_poly : &BigUint) -> (BigUint, BigUint)
{
  let mut a = a.clone();
  let mut q = BigUint::zero();
  let mut a_bits = a.bits();
  let poly_size = prime_poly.bits();
  while a_bits >= poly_size
  {
    let shift_length = a_bits - poly_size;
    add(&mut a, &(prime_poly << shift_length));
    a_bits = a.bits();
    q += BigUint::one() << shift_length;
  }
  (q, a)
}