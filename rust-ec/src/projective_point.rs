use crate::affine_point::EcPointA;
use crate::helpers::{affine_to_projective, projective_to_affine};
use crate::ECurve;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};

#[derive(Clone, Eq, PartialEq, PartialOrd)]
pub struct EcPointP {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt,
}

// todo: look here https://github.com/pornin/crrl/blob/main/src/p256.rs
impl EcPointP {
    pub fn new(x: &BigInt, y: &BigInt, z: &BigInt) -> Self {
        EcPointP {
            x: x.clone(),
            y: y.clone(),
            z: z.clone(),
        }
    }

    pub fn from_affine(a: &EcPointA) -> Self {
        affine_to_projective(a)
    }

    pub fn to_affine(&self, ec_curve: &ECurve) -> crate::Result<EcPointA> {
        projective_to_affine(ec_curve, self)
    }

    pub fn is_inf(&self) -> bool {
        self.x.is_zero() && self.y.is_one() && self.z.is_zero()
    }

    pub fn neutral() -> EcPointP {
        EcPointP {
            x: Default::default(),
            y: BigInt::one(),
            z: Default::default(),
        }
    }

    pub fn negative(&self) -> EcPointP {
        EcPointP {
            x: self.x.clone(),
            y: -self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl UpperHex for EcPointP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:X}, y: {:X} ,z: {:X}", self.x, self.y, self.z)
    }
}

impl LowerHex for EcPointP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:x}, y: {:x} ,z: {:x}", self.x, self.y, self.z)
    }
}

impl Debug for EcPointP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {} ,z: {}", self.x, self.y, self.z)
    }
}

impl Display for EcPointP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {} ,z: {}", self.x, self.y, self.z)
    }
}
