pub mod affine_point;
pub mod helpers;
pub mod projective_point;

use crate::affine_point::EcPointA;
use crate::helpers::{
    check_discriminant, inverse, projective_add, projective_mul, take_by_bigint_module,
};
use crate::projective_point::EcPointP;
use num_bigint::{BigInt, BigUint};
use num_traits::{Num, One};
use std::fmt::{Display, Formatter};

/// **ECCurve** -- represents elliptic curve in Weierstrass form
/// points satisfy the following equation
/// y^2 = x^3 + ax + b or in projective coordinates Y^{2}Z = X^{3} + aXZ^{2} + bZ^3
/// and EC discriminant has to be not equal to zero, i.e. 4a^3 + 27b^2 mod q != 0
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct ECurve {
    a: BigInt,
    b: BigInt,
    q: BigInt,
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct EcInfo {
    pub bp: EcPointP,
    /// **n** -- order of EC
    pub n: BigUint,
    pub ecurve: ECurve,
}

#[derive(Debug)]
pub struct Params {
    pub a: BigInt,
    pub b: BigInt,
    pub q: BigInt,
}

#[derive(Debug)]
pub enum PreGeneratedParams {
    P192,
    P224,
    P256,
    P384,
}

#[derive(Debug)]
pub enum EcError {
    IncorrectParameters(String),
    NonZeroDiscriminant(BigInt),
    ImpossibleToFindInverse(String),
}

pub type Result<T> = core::result::Result<T, EcError>;

impl Display for EcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EcError::IncorrectParameters(str) => format!("Can't parse EC parameters: {str}"),
                EcError::NonZeroDiscriminant(x) =>
                    format!("Discriminant doesn't equal to zero [{x} != 0]"),
                EcError::ImpossibleToFindInverse(msg) =>
                    format!("Impossible to find inverse for {msg}"),
            }
        )
    }
}

impl From<PreGeneratedParams> for Params {
    fn from(value: PreGeneratedParams) -> Self {
        Params::from(&value)
    }
}
impl From<&PreGeneratedParams> for Params {
    fn from(value: &PreGeneratedParams) -> Self {
        match value {
            PreGeneratedParams::P192 => Params {
                a: BigInt::from(-3),
                b: BigInt::from_str_radix(
                    "64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1",
                    16,
                )
                    .unwrap(),
                q: BigInt::from_str_radix(
                    "6277101735386680763835789423207666416083908700390324961279",
                    10,
                )
                    .unwrap(),
            },
            PreGeneratedParams::P224 => Params {
                a: BigInt::from(-3),
                b: BigInt::from_str_radix(
                    "b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4",
                    16,
                )
                    .unwrap(),
                q: BigInt::from_str_radix(
                    "26959946667150639794667015087019630673557916260026308143510066298881",
                    10,
                )
                    .unwrap(),
            },
            PreGeneratedParams::P256 => Params {
                a: BigInt::from(-3),
                b: BigInt::from_str_radix(
                    "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
                    16,
                )
                    .unwrap(),
                q: BigInt::from_str_radix(
                    "115792089210356248762697446949407573530086143415290314195533631308867097853951",
                    10,
                )
                    .unwrap(),
            },
            PreGeneratedParams::P384 => Params {
                a: BigInt::from(-3),
                b: BigInt::from_str_radix(
                    "b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875a",
                    16,
                )
                    .unwrap(),
                q: BigInt::from_str_radix(
                    "39402006196394479212279040100143613805079739270465446667948293404245721771496870329047266088258938001861606973112319",
                    10,
                )
                    .unwrap(),
            },
        }
    }
}

impl From<PreGeneratedParams> for EcInfo {
    fn from(value: PreGeneratedParams) -> Self {
        EcInfo::from(&value)
    }
}
impl From<&PreGeneratedParams> for EcInfo {
    fn from(value: &PreGeneratedParams) -> Self {
        match value {
            PreGeneratedParams::P192 => EcInfo {
                bp: EcPointP::from_affine(&EcPointA{
                    x: BigInt::from_str_radix( "188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012",16).unwrap(),
                    y: BigInt::from_str_radix( "07192b95ffc8da78631011ed6b24cdd573f977a11e794811",16).unwrap(),
                    is_inf: false,
                }),
                n: BigUint::from_str_radix("6277101735386680763835789423176059013767194773182842284081", 10).unwrap(),
                ecurve: ECurve::new(Params::from(value)).unwrap(),
            },
            PreGeneratedParams::P224 => {
                todo!()
            }
            PreGeneratedParams::P256 => {
                todo!()
            }
            PreGeneratedParams::P384 => {
                todo!()
            }
        }
    }
}

impl Params {
    fn check_discriminant(&self) -> Result<()> {
        check_discriminant(&self.a, &self.b, &self.q)
    }
}

impl ECurve {
    pub fn new(params: Params) -> Result<Self> {
        params.check_discriminant()?;
        let ec = ECurve {
            a: params.a,
            b: params.b,
            q: params.q,
        };
        Ok(ec)
    }

    // y^2 = x^3 + ax + b
    pub fn check_affine_point(&self, p: &EcPointA) -> bool {
        p.y.modpow(&BigInt::from(2_u8), &self.q)
            == p.x.modpow(&BigInt::from(3_u8), &self.q) + &self.a * &p.x + &self.b
    }

    // Y^{2}Z = X^{3} + aXZ^{2} + bZ^3,
    pub fn check_projective_point(&self, p: &EcPointP) -> bool {
        take_by_bigint_module(
            &((p.y.modpow(&BigInt::from(2_u8), &self.q) * &p.z) % &self.q),
            &self.q,
        ) == take_by_bigint_module(
            &((p.x.modpow(&BigInt::from(3_u8), &self.q)
                + &self.a * &p.x * p.z.modpow(&BigInt::from(2_u8), &self.q)
                + &self.b * p.z.modpow(&BigInt::from(3_u8), &self.q))
                % &self.q),
            &self.q,
        )
    }

    pub fn affine_point_add(&self, a: &EcPointA, b: &EcPointA) -> Result<EcPointA> {
        self.proj_point_add(&a.to_projective(), &b.to_projective())
            .to_affine(self)
    }
    pub fn affine_point_mul(&self, a: &EcPointA, k: &BigUint) -> Result<EcPointA> {
        self.proj_point_mul(&a.to_projective(), k).to_affine(self)
    }
    pub fn proj_point_add(&self, a: &EcPointP, b: &EcPointP) -> EcPointP {
        projective_add(self, a, b)
    }
    pub fn proj_point_mul(&self, a: &EcPointP, k: &BigUint) -> EcPointP {
        projective_mul(self, a, k)
    }

    /// **transform_proj_point** -- transforms projective point Z coordinate into 1
    pub fn transform_proj_point(&self, p: &EcPointP) -> crate::Result<EcPointP> {
        let mut p = p.clone();
        let inv = inverse(&take_by_bigint_module(&p.z, &self.q), &self.q)?;
        p.x = take_by_bigint_module(&((&p.x * &inv) % &self.q), &self.q);
        p.y = take_by_bigint_module(&((&p.y * &inv) % &self.q), &self.q);
        p.z = BigInt::one();
        Ok(p)
    }

    /// **take_by_module** -- takes all parts of the point by module
    /// i.e. -3 mod 13 = 10 (it removes minuses from point coordinates)
    pub fn take_by_module(&self, p: &EcPointP) -> EcPointP {
        EcPointP {
            x: take_by_bigint_module(&p.x, &self.q),
            y: take_by_bigint_module(&p.y, &self.q),
            z: take_by_bigint_module(&p.z, &self.q),
        }
    }
}
