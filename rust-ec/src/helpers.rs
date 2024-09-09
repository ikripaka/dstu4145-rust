use crate::affine_point::EcPointA;
use crate::projective_point::EcPointP;
use crate::{ECurve, EcError};
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

/// **projective_to_affine** -- transforms (X, Y, Z) => (X*(Z^{-1} mod q), Y*(Z^{-1} mod q))
pub fn projective_to_affine(ec: &ECurve, a: &EcPointP) -> crate::Result<EcPointA> {
    if a.is_inf() || a.z == BigInt::zero() {
        Ok(EcPointA {
            x: Default::default(),
            y: Default::default(),
            is_inf: true,
        })
    } else {
        match inverse(&a.z.to_bigint().unwrap(), &ec.q.to_bigint().unwrap()) {
            Ok(inv) => Ok(EcPointA {
                x: take_by_bigint_module(&(&a.x * &inv), &ec.q),
                y: take_by_bigint_module(&(&a.y * &inv), &ec.q),
                is_inf: false,
            }),
            Err(err) => Err(err),
        }
    }
}

// https://math.stackexchange.com/questions/1737883/convert-affine-coordinates-to-projective-coordinates
// Y^{2}Z = X^{3} + aXZ^{2} + bZ^3
pub fn affine_to_projective(a: &EcPointA) -> EcPointP {
    if a.is_inf{
        return EcPointP::neutral()
    }
    EcPointP {
        x: a.x.clone(),
        y: a.y.clone(),
        z: BigInt::one(),
    }
}

/// **check_discriminant** -- check equation: 4a^3 + 27b^2 != 0
pub fn check_discriminant(a: &BigInt, b: &BigInt, q: &BigInt) -> crate::Result<()> {
    let d = (BigInt::from(4_u8) * a.modpow(&BigInt::from(3_u8), q)
        + BigInt::from(27_u8) * b.modpow(&BigInt::from(2_u8), q))
        % q;
    if d != BigInt::zero() {
        Ok(())
    } else {
        Err(EcError::NonZeroDiscriminant(d))
    }
}

// Algorithm to find inverse by module using Extended Euclides algorithm
pub fn inverse(a: &BigInt, n: &BigInt) -> crate::Result<BigInt> {
    let mut a_mut = a.clone();
    if a >= n {
        a_mut %= n;
    }

    let mut t = BigInt::zero();
    let mut r = n.clone();
    let mut new_t = BigInt::one();
    let mut new_r = a_mut.clone();
    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        let new_t_aux = t;
        t = new_t.clone();
        new_t = new_t_aux - &quotient * &new_t;
        let new_r_aux = r; //auxiliary
        r = new_r.clone();
        new_r = new_r_aux - &quotient * &new_r;
    }
    if r > BigInt::one() {
        return Err(EcError::ImpossibleToFindInverse(format!("a: {a}, n: {n}")));
    }
    if t < BigInt::zero() {
        t += n;
    }
    Ok(t)
}

pub fn take_by_bigint_module(x: &BigInt, q: &BigInt) -> BigInt {
    if *x < BigInt::zero() {
        (q + x) % q
    } else {
        x % q
    }
}
pub fn take_by_biguint_module(x: &BigInt, q: &BigUint) -> BigInt {
    take_by_bigint_module(x, &BigInt::from(q.clone()))
}

pub(crate) fn projective_add(ec_curve: &ECurve, a: &EcPointP, b: &EcPointP) -> EcPointP {
    if a.is_inf() {
        return b.clone();
    } else if b.is_inf() {
        return a.clone();
    }

    let u_1 = (&b.y * &a.z) % &ec_curve.q;
    let u_2 = (&a.y * &b.z) % &ec_curve.q;
    let v_1 = (&b.x * &a.z) % &ec_curve.q;
    let v_2 = (&a.x * &b.z) % &ec_curve.q;

    if v_1 == v_2 {
        if u_1 == u_2 {
            return projective_double(ec_curve, a);
        } else {
            return EcPointP::neutral();
        }
    }

    let u = (&u_1 - &u_2) % &ec_curve.q;
    let v = (&v_1 - &v_2) % &ec_curve.q;
    let w = (&a.z * &b.z) % &ec_curve.q;
    let a = (&u * &u * &w - &v * &v * &v - BigInt::from(2) * &v * &v * &v_2) % &ec_curve.q;
    let x3 = (&v * &a) % &ec_curve.q;
    let y3 = (&u * (&v * &v * &v_2 - &a) - &v * &v * &v * &u_2) % &ec_curve.q;
    let z3 = (&v * &v * &v * &w) % &ec_curve.q;
    EcPointP {
        x: take_by_bigint_module(&x3, &ec_curve.q),
        y: take_by_bigint_module(&y3, &ec_curve.q),
        z: take_by_bigint_module(&z3, &ec_curve.q),
    }
}

pub(crate) fn projective_double(ec_curve: &ECurve, a: &EcPointP) -> EcPointP {
    if a.is_inf() || a.y == BigInt::zero() {
        EcPointP::neutral()
    } else {
        let two = BigInt::from(2_u8);
        let t = (&a.x * &a.x * BigInt::from(3_u8) + &ec_curve.a * &a.z * &a.z) % &ec_curve.q;
        let u = (&a.y * &a.z * &two) % &ec_curve.q;
        let v = (&u * &a.x * &a.y * &two) % &ec_curve.q;
        let w = (&t * &t - &v * &two) % &ec_curve.q;
        let rx = (&u * &w) % &ec_curve.q;
        let ry = (&t * (&v - &w) - &u * &u * &a.y * &a.y * &two) % &ec_curve.q;
        let rz = (&u * &u * &u) % &ec_curve.q;
        EcPointP {
            x: take_by_bigint_module(&rx, &ec_curve.q),
            y: take_by_bigint_module(&ry, &ec_curve.q),
            z: take_by_bigint_module(&rz, &ec_curve.q),
        }
    }
}

pub(crate) fn projective_mul(ec_curve: &ECurve, a: &EcPointP, k: &BigUint) -> EcPointP {
    let mut r = EcPointP::neutral();
    let mut tmp = a.clone();

    // from LSB to MSB
    for x in k.to_str_radix(2).into_bytes().iter().rev() {
        if *x - b'0' == 1 {
            r = ec_curve.proj_point_add(&r, &tmp)
        }
        tmp = ec_curve.proj_point_add(&tmp, &tmp)
    }
    r
}
