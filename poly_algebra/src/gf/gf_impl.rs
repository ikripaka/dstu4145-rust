use std::{fmt, mem};
use std::fmt::Formatter;
use std::ops::{Add, Mul};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::{
  helpers::{create_prime_polynomial, to_binary_be, to_lower_hex_be, to_upper_hex_be},
  gf::gf_def::{GFArithmetic, GFFactory},
  gf_arithmetic::{
    addition::add,
    multiplication::{mul},
    reduction::module_reduction,
  },
  impl_gf_for_poly
};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF163 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF167 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF173 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF179 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF191 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF233 {
  pub poly: BigUint,
  prime_poly: BigUint,
}#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF239 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF257 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF307 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF367 {
  pub poly: BigUint,
  prime_poly: BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF431 {
  pub poly: BigUint,
  prime_poly: BigUint,
}

impl_gf_for_poly!(GF163, &[163_u32, 7, 6, 3, 0]);
impl_gf_for_poly!(GF167, &[167_u32, 6, 0]);
impl_gf_for_poly!(GF173, &[173_u32, 10, 2, 1, 0]);
impl_gf_for_poly!(GF179, &[179_u32, 4, 2, 1, 0]);
impl_gf_for_poly!(GF191, &[191_u32, 9, 0]);
impl_gf_for_poly!(GF233, &[233_u32, 9, 4, 1, 0]);
impl_gf_for_poly!(GF239, &[239_u32, 15, 2, 1, 0]);
impl_gf_for_poly!(GF257, &[257_u32, 12, 0]);
impl_gf_for_poly!(GF307, &[307_u32, 8, 4, 2, 0]);
impl_gf_for_poly!(GF367, &[367_u32, 21, 0]);
impl_gf_for_poly!(GF431, &[431_u32, 5, 3, 1, 0]);
