use crate::helpers::{affine_to_projective, projective_to_affine};
use crate::projective_point::EcPointP;
use crate::ECurve;
use num_bigint::{BigInt};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EcPointA {
    pub(crate) x: BigInt,
    pub(crate) y: BigInt,
    pub(crate) is_inf: bool,
}

impl EcPointA {
    pub fn new(x: &BigInt, y: &BigInt) -> Self {
        EcPointA {
            x: x.clone(),
            y: y.clone(),
            is_inf: false, //check_affine_point_for_inf(x, y),
        }
    }

    pub fn from_projective(a: &EcPointP, ec_curve: &ECurve) -> crate::Result<Self> {
        projective_to_affine(ec_curve, a)
    }

    pub fn to_projective(&self) -> EcPointP {
        affine_to_projective(self)
    }

    pub fn negative(&self) -> EcPointA {
        EcPointA {
            x: self.x.clone(),
            y: -self.y.clone(),
            is_inf: self.is_inf,
        }
    }

    pub fn get_x(&self) -> BigInt{
        self.x.clone()
    }
    pub fn get_y(&self) -> BigInt{
        self.y.clone()
    }

    pub fn is_inf(&self) -> bool{
        self.is_inf
    }
}
