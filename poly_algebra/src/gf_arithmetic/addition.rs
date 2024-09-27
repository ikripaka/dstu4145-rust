use num_bigint::BigUint;
use num_traits::One;
use crate::gf_arithmetic::reduction::module_reduction;

pub fn add(a : &mut BigUint, b : &BigUint) { *a ^= b; }
