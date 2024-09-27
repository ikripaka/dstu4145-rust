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
  impl_gf_for_poly, impl_gf_display, impl_gf_conversions, impl_obj_safe_gf_for_poly,
};
use crate::gf::gf_def::{GFArithmeticObjSafe, GFDisplay, GFFactoryObjSafe, GFGetters};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF3
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF163
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF167
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF173
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF179
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF191
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF233
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF239
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF257
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF307
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF367
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GF431
{
  pub poly : BigUint,
  prime_poly : BigUint,
}
const GF3_PRIME_POLY : [u32; 3] = [3, 1, 0];
const GF239_PRIME_POLY : [u32; 5] = [239_u32, 15, 2, 1, 0];
const GF163_PRIME_POLY : [u32; 5] = [163_u32, 7, 6, 3, 0];
const GF167_PRIME_POLY : [u32; 3] = [167_u32, 6, 0];
const GF173_PRIME_POLY : [u32; 5] = [173_u32, 10, 2, 1, 0];
const GF179_PRIME_POLY : [u32; 5] = [179_u32, 4, 2, 1, 0];
const GF191_PRIME_POLY : [u32; 3] = [191_u32, 9, 0];
const GF233_PRIME_POLY : [u32; 5] = [233_u32, 9, 4, 1, 0];
const GF257_PRIME_POLY : [u32; 3] = [257_u32, 12, 0];
const GF307_PRIME_POLY : [u32; 5] = [307_u32, 8, 4, 2, 0];
const GF367_PRIME_POLY : [u32; 3] = [367_u32, 21, 0];
const GF431_PRIME_POLY : [u32; 5] = [431_u32, 5, 3, 1, 0];

impl_gf_for_poly!(GF3, &GF3_PRIME_POLY);
impl_gf_display!(GF3);
impl_obj_safe_gf_for_poly!(GF163, &[3, 1, 0]);
impl_gf_conversions!(GF3);

impl_gf_for_poly!(GF239, &GF239_PRIME_POLY);
impl_gf_display!(GF239);
impl_obj_safe_gf_for_poly!(GF239, &[239_u32, 15, 2, 1, 0]);
impl_gf_conversions!(GF239);

impl_gf_for_poly!(GF163, &GF163_PRIME_POLY);
impl_gf_display!(GF163);
impl_obj_safe_gf_for_poly!(GF3, &[163_u32, 7, 6, 3, 0]);
impl_gf_conversions!(GF163);

impl_gf_for_poly!(GF167, &GF167_PRIME_POLY);
impl_gf_display!(GF167);
impl_obj_safe_gf_for_poly!(GF167, &[167_u32, 6, 0]);
impl_gf_conversions!(GF167);

impl_gf_for_poly!(GF173, &GF173_PRIME_POLY);
impl_gf_display!(GF173);
impl_obj_safe_gf_for_poly!(GF173, &[173_u32, 10, 2, 1, 0]);
impl_gf_conversions!(GF173);

impl_gf_for_poly!(GF179, &GF179_PRIME_POLY);
impl_gf_display!(GF179);
impl_obj_safe_gf_for_poly!(GF179, &[179_u32, 4, 2, 1, 0]);
impl_gf_conversions!(GF179);

impl_gf_for_poly!(GF191, &GF191_PRIME_POLY);
impl_gf_display!(GF191);
impl_obj_safe_gf_for_poly!(GF191, &[191_u32, 9, 0]);
impl_gf_conversions!(GF191);

impl_gf_for_poly!(GF233, &GF233_PRIME_POLY);
impl_gf_display!(GF233);
impl_obj_safe_gf_for_poly!(GF233, &[233_u32, 9, 4, 1, 0]);
impl_gf_conversions!(GF233);

impl_gf_for_poly!(GF257, &GF257_PRIME_POLY);
impl_gf_display!(GF257);
impl_obj_safe_gf_for_poly!(GF257, &[257_u32, 12, 0]);
impl_gf_conversions!(GF257);

impl_gf_for_poly!(GF307, &GF307_PRIME_POLY);
impl_gf_display!(GF307);
impl_obj_safe_gf_for_poly!(GF307, &[307_u32, 8, 4, 2, 0]);
impl_gf_conversions!(GF307);

impl_gf_for_poly!(GF367, &GF367_PRIME_POLY);
impl_gf_display!(GF367);
impl_obj_safe_gf_for_poly!(GF367, &[367_u32, 21, 0]);
impl_gf_conversions!(GF367);

impl_gf_for_poly!(GF431, &GF431_PRIME_POLY);
impl_gf_display!(GF431);
impl_obj_safe_gf_for_poly!(GF431, &[431_u32, 5, 3, 1, 0]);
impl_gf_conversions!(GF431);
