#[cfg(test)]
mod tests {
    use num_bigint::{BigInt, BigUint};
    use num_traits::{Num, One};
    use rust_ec::affine_point::EcPointA;
    use rust_ec::projective_point::EcPointP;
    use rust_ec::{ECurve, Params, PreGeneratedParams};

    #[test]
    fn testing_ec_creation() {
        assert!(ECurve::new(Params {
            a: BigInt::from(1_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(7_u8),
        })
        .is_ok());

        //todo: come up with example equal to zero
        assert!(ECurve::new(Params {
            a: BigInt::from(1_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(7_u8),
        })
        .is_ok());

        assert!(ECurve::new(Params {
            a: BigInt::from(3_u8),
            b: BigInt::from(7_u8),
            q: BigInt::from(949_u64),
        })
        .is_ok());

        assert!(ECurve::new(Params {
            a: BigInt::from(5_u8),
            b: BigInt::from(3_u8),
            q: BigInt::from(31_u8),
        })
        .is_ok()); // = -1

        assert!(ECurve::new(Params {
            a: BigInt::from(8_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(11_u8),
        })
        .is_ok());

        assert!(ECurve::new(Params {
            a: BigInt::from(1_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(31_u8),
        })
        .is_err());
    }

    #[test]
    fn negative() {
        let affine_point = EcPointA::new(&BigInt::from(11_u8), &BigInt::from(22_u8));
        assert_eq!(
            affine_point.negative(),
            EcPointA::new(&BigInt::from(11_u8), &BigInt::from(-22_i8))
        );

        let projective_point = EcPointP::new(
            &BigInt::from(11_u8),
            &BigInt::from(22_u8),
            &BigInt::from(33_u8),
        );
        assert_eq!(
            projective_point.negative(),
            EcPointP::new(
                &BigInt::from(11_u8),
                &BigInt::from(-22_i8),
                &BigInt::from(33_i8),
            )
        );
    }

    #[test]
    fn ec_add() {
        let curve = ECurve::new(Params {
            a: BigInt::from(11_u8),
            b: BigInt::from(7_u8),
            q: BigInt::from(13_u8),
        })
        .unwrap();

        let p1 = EcPointP::new(&BigInt::from(6), &BigInt::from(4), &BigInt::from(1));
        let p2 = curve.proj_point_add(&p1, &p1);
        // EcPointP::new(&BigInt::from(10),&BigInt::from(8),&BigInt::from(1));
        assert_eq!(
            curve
                .transform_proj_point(&curve.proj_point_add(&p1, &p2))
                .unwrap(),
            EcPointP::new(&BigInt::from(11), &BigInt::from(4), &BigInt::from(1))
        );
        assert_eq!(
            curve
                .transform_proj_point(&curve.proj_point_add(&p2, &p2))
                .unwrap(),
            EcPointP::new(&BigInt::from(9), &BigInt::from(9), &BigInt::from(1))
        );

        let x_p3 = curve.proj_point_add(&p1, &p2);
        let x_p4 = curve.proj_point_add(&p2, &p2);
        let x_p5 = curve.proj_point_add(&x_p4, &p1);
        let x_p6 = curve.proj_point_add(&x_p5, &p1);
        let x_p7 = curve.proj_point_add(&x_p6, &p1);
        let x_p8 = curve.proj_point_add(&x_p7, &p1);
        let x_p9 = curve.proj_point_add(&x_p8, &p1);
        let x_p10 = curve.proj_point_add(&x_p9, &p1);
        let x_p11 = curve.proj_point_add(&x_p10, &p1);
        let x_p12 = curve.proj_point_add(&x_p11, &p1);

        println!("p1:{:?},\np2:{:?},\np3:{:?},\np4:{:?},\np5:{:?},\np6:{:?},\np7:{:?},\np8:{:?},\np9:{:?},\np10:{:?},\np11:{:?},\n ",
                 curve.transform_proj_point(&p1),
                 curve.transform_proj_point(&p2),
                 curve.transform_proj_point(&x_p3),
                 curve.transform_proj_point(&x_p4),
                 curve.transform_proj_point(&x_p5),
                 curve.transform_proj_point(&x_p6),
                 curve.transform_proj_point(&x_p7),
                 curve.transform_proj_point(&x_p8),
                 curve.transform_proj_point(&x_p9),
                 curve.transform_proj_point(&x_p10),
                 x_p11);

        // assert!(p12.is_inf(), "{p12:?}");

        let p3 = curve.proj_point_add(&p1, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p3).unwrap(),
            curve.transform_proj_point(&p3).unwrap()
        );
        let p4 = curve.proj_point_add(&p2, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p4).unwrap(),
            curve.transform_proj_point(&p4).unwrap()
        );
        let p5 = curve.proj_point_add(&p3, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p5).unwrap(),
            curve.transform_proj_point(&p5).unwrap()
        );
        let p6 = curve.proj_point_add(&p5, &p1);
        assert_eq!(
            curve.transform_proj_point(&x_p6).unwrap(),
            curve.transform_proj_point(&p6).unwrap()
        );
        let p7 = curve.proj_point_add(&p6, &p1);
        assert_eq!(
            curve.transform_proj_point(&x_p7).unwrap(),
            curve.transform_proj_point(&p7).unwrap()
        );
        let p8 = curve.proj_point_add(&p4, &p4);
        assert_eq!(
            curve.transform_proj_point(&x_p8).unwrap(),
            curve.transform_proj_point(&p8).unwrap()
        );
        let p9 = curve.proj_point_add(&p5, &p4);
        assert_eq!(
            curve.transform_proj_point(&x_p9).unwrap(),
            curve.transform_proj_point(&p9).unwrap()
        );
        let p10 = curve.proj_point_add(&p8, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p10).unwrap(),
            curve.transform_proj_point(&p10).unwrap()
        );
        let p11 = curve.proj_point_add(&p8, &p3);
        assert_eq!(&x_p11, &p11);
        let p12 = curve.proj_point_add(&p8, &p4);
        assert_eq!(
            curve.transform_proj_point(&x_p12).unwrap(),
            curve.transform_proj_point(&p12).unwrap()
        );

        assert!(
            p11.is_inf() && x_p11.is_inf(),
            "p11: {p11:?}, x_p11: {x_p11:?}"
        );
        {
            let p5 = curve.transform_proj_point(&p5).unwrap();
            let p6 = curve.transform_proj_point(&p6).unwrap();
            assert_eq!(
                curve.proj_point_add(&p5, &p6),
                EcPointP::neutral(),
                "{}",
                format!(
                    "P6: {p6:?} (or {:?}) does it on EC {},\nP5: {p5:?} (or {:?}) does it on EC {}",
                    curve.transform_proj_point(&p6),
                    curve.check_projective_point(&p6),
                    curve.transform_proj_point(&p5),
                    curve.check_projective_point(&p5)
                )
            );
        }
        assert_eq!(
            curve.proj_point_add(&p6, &curve.proj_point_add(&p8, &p8)),
            EcPointP::neutral(),
            "{}",
            {
                let p16 = curve.proj_point_add(&p8, &p8);
                format!("failed to add 6P + 16P = 22P = 0P, \nP6: {p6:?} (or {:?}) does it on EC {},\nP16: {:?} (or {:?}) does it on EC {}",
                        curve.transform_proj_point(&p6),
                        curve.check_projective_point(&p6),
                        &p16,
                        curve.transform_proj_point(&p16),
                        curve.check_projective_point(&p16))
            }
        );

        let p12 = curve.proj_point_add(&p1, &p11);
        let p13 = curve.proj_point_add(&p12, &p1);
        assert_eq!(
            curve.transform_proj_point(&p12).unwrap(),
            curve.transform_proj_point(&p1).unwrap()
        );
        assert_eq!(
            curve.transform_proj_point(&p13).unwrap(),
            curve.transform_proj_point(&p2).unwrap()
        );

        // doubling
        let p12_d = curve.proj_point_add(&p12, &p12);
        assert_eq!(
            curve.transform_proj_point(&p12_d).unwrap(),
            curve.transform_proj_point(&p2).unwrap()
        );
        let p10_d = curve.proj_point_add(&p10, &p10);
        assert_eq!(
            curve.transform_proj_point(&p10_d).unwrap(),
            curve.transform_proj_point(&p9).unwrap()
        );
        let p6_d = curve.proj_point_add(&p6, &p6);
        assert_eq!(
            curve.transform_proj_point(&p6_d).unwrap(),
            curve.transform_proj_point(&p12).unwrap()
        );

        let p8_d = curve.proj_point_add(&p4, &p4);
        assert_eq!(
            curve.transform_proj_point(&p8_d).unwrap(),
            curve.transform_proj_point(&p8).unwrap()
        );
        let p16_d = curve.proj_point_add(&p8_d, &p8_d);
        assert_eq!(
            curve.transform_proj_point(&p16_d).unwrap(),
            curve.transform_proj_point(&p5).unwrap()
        );
        let p32_d = curve.proj_point_add(&p16_d, &p16_d);
        assert_eq!(
            curve.transform_proj_point(&p32_d).unwrap(),
            curve.transform_proj_point(&p10).unwrap()
        );
        let p64_d = curve.proj_point_add(&p32_d, &p32_d);
        assert_eq!(
            curve.transform_proj_point(&p64_d).unwrap(),
            curve.transform_proj_point(&p9).unwrap()
        );

        // adding negative value
        let p12_n = curve.proj_point_add(&p12, &p12.negative());
        assert!(p12_n.is_inf(), "p12_n: {p12:?}");
        let p10_n = curve.proj_point_add(&p10, &p10.negative());
        assert!(p10_n.is_inf(), "p10_n: {p10_n:?}");
        let p6_n = curve.proj_point_add(&p6, &p6.negative());
        assert!(p6_n.is_inf(), "p6_n: {p6_n:?}");
        let p8_n = curve.proj_point_add(&p8, &p8.negative());
        assert!(p8_n.is_inf(), "p8_n: {p8_n:?}");

        let p22_d = curve.proj_point_add(&p10, &p12);
        let p22_m = curve.proj_point_mul(&p1, &BigUint::from(22_u8));
        assert_eq!(
            curve.transform_proj_point(&p10_d).unwrap(),
            curve.transform_proj_point(&p9).unwrap()
        );
    }

    #[test]
    fn ec_mul() {
        let curve = ECurve::new(Params {
            a: BigInt::from(11_u8),
            b: BigInt::from(7_u8),
            q: BigInt::from(13_u8),
        })
        .unwrap();
        let order = BigUint::from(11_u8);

        let p1 = EcPointP::new(&BigInt::from(6), &BigInt::from(4), &BigInt::from(1));
        let p2 = curve.proj_point_add(&p1, &p1);
        let p3 = curve.proj_point_add(&p1, &p2);
        let p4 = curve.proj_point_add(&p2, &p2);
        let p5 = curve.proj_point_add(&p4, &p1);
        let p6 = curve.proj_point_add(&p5, &p1);
        let p7 = curve.proj_point_add(&p6, &p1);
        let p8 = curve.proj_point_add(&p7, &p1);
        let p9 = curve.proj_point_add(&p8, &p1);
        let p10 = curve.proj_point_add(&p9, &p1);
        let p11 = curve.proj_point_add(&p10, &p1);

        let p2_mul = curve.proj_point_mul(&p1, &BigUint::from(2_u8));
        assert_eq!(
            curve.transform_proj_point(&p2_mul).unwrap(),
            curve.transform_proj_point(&p2).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p3_mul = curve.proj_point_mul(&p1, &BigUint::from(3_u8));
        assert_eq!(
            curve.transform_proj_point(&p3_mul).unwrap(),
            curve.transform_proj_point(&p3).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p4_mul = curve.proj_point_mul(&p1, &BigUint::from(4_u8));
        assert_eq!(
            curve.transform_proj_point(&p4_mul).unwrap(),
            curve.transform_proj_point(&p4).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p5_mul = curve.proj_point_mul(&p1, &BigUint::from(5_u8));
        assert_eq!(
            curve.transform_proj_point(&p5_mul).unwrap(),
            curve.transform_proj_point(&p5).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p6_mul = curve.proj_point_mul(&p1, &BigUint::from(6_u8));
        assert_eq!(
            curve.transform_proj_point(&p6_mul).unwrap(),
            curve.transform_proj_point(&p6).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p7_mul = curve.proj_point_mul(&p1, &BigUint::from(7_u8));
        assert_eq!(
            curve.transform_proj_point(&p7_mul).unwrap(),
            curve.transform_proj_point(&p7).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p8_mul = curve.proj_point_mul(&p1, &BigUint::from(8_u8));
        assert_eq!(
            curve.transform_proj_point(&p8_mul).unwrap(),
            curve.transform_proj_point(&p8).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p9_mul = curve.proj_point_mul(&p1, &BigUint::from(9_u8));
        assert_eq!(
            curve.transform_proj_point(&p9_mul).unwrap(),
            curve.transform_proj_point(&p9).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p10_mul = curve.proj_point_mul(&p1, &BigUint::from(10_u8));
        assert_eq!(
            curve.transform_proj_point(&p10_mul).unwrap(),
            curve.transform_proj_point(&p10).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );

        let p11_mul = curve.proj_point_mul(&p1, &order);
        assert_eq!(p11_mul, EcPointP::neutral());

        let p12_mul = curve.proj_point_mul(&p1, &BigUint::from(12_u8));
        assert_eq!(
            curve.transform_proj_point(&p12_mul).unwrap(),
            curve.transform_proj_point(&p1).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p22_mul = curve.proj_point_mul(&p1, &BigUint::from(22_u8));
        assert_eq!(
            &p22_mul,
            &EcPointP::neutral(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p37_mul = curve.proj_point_mul(&p1, &BigUint::from(37_u8));
        assert_eq!(
            curve.transform_proj_point(&p37_mul).unwrap(),
            curve.transform_proj_point(&p4).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p57_mul = curve.proj_point_mul(&p1, &BigUint::from(57_u8));
        assert_eq!(
            curve.transform_proj_point(&p57_mul).unwrap(),
            curve.transform_proj_point(&p2).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p75_mul = curve.proj_point_mul(&p1, &BigUint::from(75_u8));
        assert_eq!(
            curve.transform_proj_point(&p75_mul).unwrap(),
            curve.transform_proj_point(&p9).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
        let p1890_mul = curve.proj_point_mul(&p1, &BigUint::from(1890_u32));
        assert_eq!(
            curve.transform_proj_point(&p1890_mul).unwrap(),
            curve.transform_proj_point(&p9).unwrap(),
            "p2_mul: {p2_mul:?}, p2: {p2:?}"
        );
    }

    #[test]
    fn p192() {
        let curve = ECurve::new(Params::from(PreGeneratedParams::P192)).unwrap();
        let g = EcPointP::new(
            &BigInt::from_str_radix("188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012", 16)
                .unwrap(),
            &BigInt::from_str_radix("07192b95ffc8da78631011ed6b24cdd573f977a11e794811", 16)
                .unwrap(),
            &BigInt::one(),
        );

        let order = BigUint::from_str_radix(
            "6277101735386680763835789423176059013767194773182842284081",
            10,
        )
        .unwrap();

        let g2 = curve
            .transform_proj_point(&curve.proj_point_add(&g, &g))
            .unwrap();
        let g3 = curve
            .transform_proj_point(&curve.proj_point_add(&g2, &g))
            .unwrap();
        let g_rand1 = curve.transform_proj_point(&curve.proj_point_mul(&g, &BigUint::from_str_radix("91525383471184876357591255416236519458931172413040567087697243677776850892863", 10).unwrap())).unwrap();
        let g_rand2 = curve.transform_proj_point(&curve.proj_point_mul(&g, &BigUint::from_str_radix("85746756882674469322490085946219637000383264126530953421180950914743422676016", 10).unwrap())).unwrap();
        let g_rand3 = curve.transform_proj_point(&curve.proj_point_mul(&g, &BigUint::from_str_radix("64521382629758530421575792491099066743806501416864342394520396229811324821000", 10).unwrap())).unwrap();
        let g_n = curve.proj_point_mul(&g, &order);

        // https://asecuritysite.com/ecc/p192p
        assert_eq!(format!("{g2:?}"), "x: 5369744403678710563432458361254544170966096384586764429448, y: 5429234379789071039750654906915254128254326554272718558123 ,z: 1");
        assert_eq!(format!("{g3:?}"), "x: 2915109630280678890720206779706963455590627465886103135194, y: 2946626711558792003980654088990112021985937607003425539581 ,z: 1");
        assert_eq!(format!("{g_rand1:?}"), "x: 292331997177533098389774896041635086638489511152398301356, y: 4824213608734868875629504923537734196035632611242225204508 ,z: 1");
        assert_eq!(format!("{g_rand2:?}"), "x: 1376758142206554388989594926457465810765436913244250629959, y: 881456195096573090089390360023938867567244399858482943796 ,z: 1");
        assert_eq!(format!("{g_rand3:?}"), "x: 310037299751269182313187755810645367178495862917706655462, y: 37078589918316711423088866017061622775521267879192452920 ,z: 1");
        assert_eq!(g_n, EcPointP::neutral());

        assert!(
            curve.check_projective_point(&g)
                && curve.check_projective_point(&g2)
                && curve.check_projective_point(&g3)
                && curve.check_projective_point(&g_rand1)
                && curve.check_projective_point(&g_rand2)
                && curve.check_projective_point(&g_rand3)
        )
    }
}
