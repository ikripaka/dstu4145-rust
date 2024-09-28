use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use poly_algebra::gf::gf_def::GF239;
use poly_algebra::helpers::to_binary_be;

fn create_prime_polynomial<T : AsRef<[u32]>>(degs : &T) -> BigUint
{
  let mut prime_poly = BigUint::zero();
  for deg in degs.as_ref()
  {
    prime_poly ^= BigUint::one() << *deg;
  }
  prime_poly
}

fn main()
{
  {
    let a = GF239::from(create_prime_polynomial(&[239, 15, 3, 2, 1, 0]));
    println!("{}", to_binary_be(&a.poly))
    // let b = GF239::from(BigUint::from_str_radix("CB43707C35204EF3C67CA42A7D756D70E1C9BA62D2CB43707C35204EF3C67CA56D70E1C9BA62D2CB4FFF", 16).unwrap());
    // let x = &b * &a;
    // assert_eq!( &b * &a, &a * &b)
  }

  // let mut x = BigUint::from_str_radix("10001001", 2).unwrap();
  // let p = BigUint::from_str_radix("111", 2).unwrap();
  // module_reduction(&mut x, &p);
  // println!("{}", to_binary_be(&x))
}
// 1101110000011111000011010100100000010011101111001111000110011111001010010000101010011111010111010101101101011100001110000111001101100100000010011000001001010000101010011110010101000100001100010110111010110100100110000011111110100111010010

// 10110101101111100011011111010101100000111010000000011001001101100100101100111110110010101100110100100001100110111000011010100100000010011101111001111000110011111001010010000101010011111010111010101101101011100001110000111001001101110100110
