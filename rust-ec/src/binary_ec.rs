use std::marker::PhantomData;
use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use poly_algebra::gf::{GFArithmetic, GF163, GF167, GF173, GF179, GF191, GF233, GF257, GF307, GF367, GF431};
use crate::affine_point::AffinePoint;
use crate::helpers::{pack_affine_point, unpack_affine_point};


/// Koblitz elliptic curve over binary field that is represented by equation:
/// $y^2 + xy = x^3 + Ax^2 + B$, where $A, B \in GF(2^m)$, $A = {0, 1}$, $B != 0$.
/// As you would expect this [BinaryEC] struct you can use with `GF` fields declared
/// in the `poly-algebra` library.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct BinaryEC<T>
{
  pub(crate) a : ACoefficient<T>,
  pub(crate) b : T,
  pub(crate) bp : AffinePoint<T>,
  pub(crate) n : BigUint,
}

/// Represents `A` coefficient in the EC equation, exactly 0 or 1 values.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ACoefficient<T>
{
  Zero(PhantomData<T>),
  One(PhantomData<T>),
}
impl<'a, T : GFArithmetic<'a>> ACoefficient<T>
{
  pub fn as_field_el(&self) -> T { T::from(self.as_biguint()) }

  pub fn as_biguint(&self) -> BigUint
  {
    match self
    {
      ACoefficient::Zero(_) => BigUint::zero(),
      ACoefficient::One(_) => BigUint::one(),
    }
  }
}

impl BinaryEC<GF163>
{
  /// Generates EC over [GF163] field.
  pub fn generate_m163_pb_curve() -> BinaryEC<GF163>
  {
    BinaryEC::<GF163> {
      a : ACoefficient::One(PhantomData::<GF163>),
      b : GF163::from(BigUint::from_str_radix("5FF6108462A2DC8210AB403925E638A19C1455D21", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF163::from(BigUint::from_str_radix("2E2F85F5DD74CE983A5C4237229DAF8A3F35823BE", 16).unwrap()),
        y : GF163::from(BigUint::from_str_radix("3826F008A8C51D7B95284D9D03FF0E00CE2CD723A", 16).unwrap()),
      },
      n : BigUint::from_str_radix("400000000000000000002BEC12BE2262D39BCF14D", 16).unwrap(),
    }
  }

  /// Generates EC over [GF163] field exactly created for test purposes.
  pub fn generate_m163_pb_curve_from_examples() -> BinaryEC<GF163>
  {
    BinaryEC::<GF163> {
      a : ACoefficient::One(PhantomData::<GF163>),
      b : GF163::from(BigUint::from_str_radix("5FF6108462A2DC8210AB403925E638A19C1455D21", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF163::from(BigUint::from_str_radix("72D867F93A93AC27DF9FF01AFFE74885C8C540420", 16).unwrap()),
        y : GF163::from(BigUint::from_str_radix("0224A9C3947852B97C5599D5F4AB81122ADC3FD9B", 16).unwrap()),
      },
      n : BigUint::from_str_radix("400000000000000000002BEC12BE2262D39BCF14D", 16).unwrap(),
    }
  }
}

impl BinaryEC<GF167>
{
  /// Generates EC over [GF167] field.
  pub fn generate_m167_pb_curve() -> BinaryEC<GF167>
  {
    BinaryEC::<GF167> {
      a : ACoefficient::One(PhantomData::<GF167>),
      b : GF167::from(BigUint::from_str_radix("6EE3CEEB230811759F20518A0930F1A4315A827DAC", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF167::from(BigUint::from_str_radix("7A1F6653786A68192803910A3D30B2A2018B21CD54", 16).unwrap()),
        y : GF167::from(BigUint::from_str_radix("5F49EB26781C0EC6B8909156D98ED435E45FD59918", 16).unwrap()),
      },
      n : BigUint::from_str_radix("3FFFFFFFFFFFFFFFFFFFFFB12EBCC7D7F29FF7701F", 16).unwrap(),
    }
  }
}
impl BinaryEC<GF173>
{
  /// Generates EC over [GF173] field.
  pub fn generate_m173_pb_curve() -> BinaryEC<GF173>
  {
    BinaryEC::<GF173> {
      a : ACoefficient::Zero(PhantomData::<GF173>),
      b : GF173::from(BigUint::from_str_radix("108576C80499DB2FC16EDDF6853BBB278F6B6FB437D9", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF173::from(BigUint::from_str_radix("4D41A619BCC6EADF0448FA22FAD567A9181D37389CA", 16).unwrap()),
        y : GF173::from(BigUint::from_str_radix("10B51CC12849B234C75E6DD2028BF7FF5C1CE0D991A1", 16).unwrap()),
      },
      n : BigUint::from_str_radix("800000000000000000000189B4E67606E3825BB2831", 16).unwrap(),
    }
  }
}
impl BinaryEC<GF179>
{
  /// Generates EC over [GF179] field.
  pub fn generate_m179_pb_curve() -> BinaryEC<GF179>
  {
    BinaryEC::<GF179> {
      a : ACoefficient::One(PhantomData::<GF179>),
      b : GF179::from(BigUint::from_str_radix("4A6E0856526436F2F88DD07A341E32D04184572BEB710", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF179::from(BigUint::from_str_radix("6BA06FE51464B2BD26DC57F48819BA9954667022C7D03", 16).unwrap()),
        y : GF179::from(BigUint::from_str_radix("25FBC363582DCEC065080CA8287AAFF09788A66DC3A9E", 16).unwrap()),
      },
      n : BigUint::from_str_radix("3FFFFFFFFFFFFFFFFFFFFFFB981960435FE5AB64236EF", 16).unwrap(),
    }
  }
}
impl BinaryEC<GF191>
{
  /// Generates EC over [GF191] field.
  pub fn generate_m191_pb_curve() -> BinaryEC<GF191>
  {
    BinaryEC::<GF191> {
      a : ACoefficient::One(PhantomData::<GF191>),
      b : GF191::from(BigUint::from_str_radix("7BC86E2102902EC4D5890E8B6B4981ff27E0482750FEFC03", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF191::from(BigUint::from_str_radix("714114B762F2FF4A7912A6D2AC58B9B5C2FCFE76DAEB7129", 16).unwrap()),
        y : GF191::from(BigUint::from_str_radix("29C41E568B77C617EFE5902F11DB96FA9613CD8D03DB08DA", 16).unwrap()),
      },
      n : BigUint::from_str_radix("40000000000000000000000069A779CAC1DABC6788F7474F", 16).unwrap(),
    }
  }
}
impl BinaryEC<GF233>
{
  /// Generates EC over [GF233] field.
  pub fn generate_m233_pb_curve() -> BinaryEC<GF233>
  {
    BinaryEC::<GF233> {
      a : ACoefficient::One(PhantomData::<GF233>),
      b : GF233::from(BigUint::from_str_radix("06973B15095675534C7CF7E64A21BD54EF5DD3B8A0326AA936ECE454D2C", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF233::from(BigUint::from_str_radix("3FCDA526B6CDF83BA1118DF35B3C31761D3545F32728D003EEB25EFE96", 16).unwrap()),
        y : GF233::from(BigUint::from_str_radix("9CA8B57A934C54DEEDA9E54A7BBAD95E3B2E91C54D32BE0B9DF96D8D35", 16).unwrap()),
      },
      n : BigUint::from_str_radix("1000000000000000000000000000013E974E72F8A6922031D2603CFE0D7", 16).unwrap(),
    }
  }
}
impl BinaryEC<GF257>
{
  /// Generates EC over [GF257] field.
  pub fn generate_m257_pb_curve() -> BinaryEC<GF257>
  {
    BinaryEC::<GF257> {
      a : ACoefficient::Zero(PhantomData::<GF257>),
      b : GF257::from(BigUint::from_str_radix("1CEF494720115657E18F938D7A7942394FF9425C1458C57861F9EEA6ADBE3BE10", 16).unwrap()),
      bp : AffinePoint::Point {
        x : GF257::from(
          BigUint::from_str_radix("02A29EF207D0E9B6C55CD260B306C7E007AC491CA1B10C62334A9E8DCD8D20FB7", 16).unwrap(),
        ),
        y : GF257::from(
          BigUint::from_str_radix("10686D41FF744D4449FCCF6D8EEA03102E6812C93A9D60B978B702CF156D814EF", 16).unwrap(),
        ),
      },
      n : BigUint::from_str_radix("800000000000000000000000000000006759213AF182E987D3E17714907D470D", 16).unwrap(),
    }
  }
}
impl BinaryEC<GF307>
{
  /// Generates EC over [GF307] field.
  pub fn generate_m307_pb_curve() -> BinaryEC<GF307>
  {
    BinaryEC::<GF307> {
      a : ACoefficient::One(PhantomData::<GF307>),
      b : GF307::from(
        BigUint::from_str_radix(
          "393C7F7D53666B5054B5E6C6D3DE94F4296C0C599E2E2E241050DF18B6090BDC90186904968BB",
          16,
        )
        .unwrap(),
      ),
      bp : AffinePoint::Point {
        x : GF307::from(
          BigUint::from_str_radix(
            "216EE8B189D291A0224984C1E92F1D16BF75CCD825A087A239B276D3167743C52C02D6E7232AA",
            16,
          )
          .unwrap(),
        ),
        y : GF307::from(
          BigUint::from_str_radix(
            "5D9306BACD22B7FAEB09D2E049C6E2866C5D1677762A8F2F2DC9A11C7F7BE8340AB2237C7F2A0",
            16,
          )
          .unwrap(),
        ),
      },
      n : BigUint::from_str_radix(
        "3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC079C2F3825DA70D390FBBA588D4604022B7B7",
        16,
      )
      .unwrap(),
    }
  }
}
impl BinaryEC<GF367>
{
  /// Generates EC over [GF367] field.
  pub fn generate_m367_pb_curve() -> BinaryEC<GF367>
  {
    BinaryEC::<GF367> {
      a : ACoefficient::One(PhantomData::<GF367>),
      b : GF367::from(
        BigUint::from_str_radix(
          "43FC8AD242B0B7A6F3D1627AD5654447556B47BF6AA4A64B0C2AFE42CADAB8F93D92394C79A79755437B56995136",
          16,
        )
        .unwrap(),
      ),
      bp : AffinePoint::Point {
        x : GF367::from(
          BigUint::from_str_radix(
            "324A6EDDD512F08C49A99AE0D3F961197A76413E7BE81A400CA681E09639B5FE12E59A109F78BF4A373541B3B9A1",
            16,
          )
          .unwrap(),
        ),
        y : GF367::from(
          BigUint::from_str_radix(
            "1AB597A5B4477F59E39539007C7F977D1A567B92B043A49C6B61984C3FE3481AAF454CD41BA1F051626442B3C10",
            16,
          )
          .unwrap(),
        ),
      },
      n : BigUint::from_str_radix(
        "40000000000000000000000000000000000000000000009C300B75A3FA824F22428FD28CE8812245EF44049B2D49",
        16,
      )
      .unwrap(),
    }
  }
}
impl BinaryEC<GF431>
{
  /// Generates EC over [GF431] field.
  pub fn generate_m431_pb_curve() -> BinaryEC<GF431>
  {
    BinaryEC::<GF431> {
      a : ACoefficient::One(PhantomData::<GF431>),
      b : GF431::from(
        BigUint::from_str_radix(
          "03CE10490F6A708FC26DFE8C3D27C4F94E690134D5BFF988D8D28AAEAEDE975936C66BAC536B18AE2DC312CA493117DAA469C640CAF3",
          16,
        )
        .unwrap(),
      ),
      bp : AffinePoint::Point {
        x : GF431::from(
          BigUint::from_str_radix(
            "1A62BA79D98133A16BBAE7ED9A8E03C32E0824D57AEF72F88986874E5AAE49C27BED49A2A95058068426C2171E99FD3B43C5947C857D",
            16,
          )
          .unwrap(),
        ),
        y : GF431::from(
          BigUint::from_str_radix(
            "70B5E1E14031C1F70BBEFE96BDDE66F451754B4CA5F48DA241F331AA396B8D1839A855C1769B1EA14BA53308B5E2723724E090E02DB9",
            16,
          )
          .unwrap(),
        ),
      },
      n : BigUint::from_str_radix(
        "3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFBA3175458009A8C0A724F02F81AA8A1FCBAF80D90C7A95110504CF",
        16,
      )
      .unwrap(),
    }
  }
}

impl<'a, T : GFArithmetic<'a>> BinaryEC<T>
{
  /// Function gets `A` coefficient from certain EC variant.
  pub fn get_a(&self) -> ACoefficient<T> { self.a.clone() }

  /// Function gets reference to `A` coefficient from certain EC variant.
  pub fn get_ref_a(&self) -> &ACoefficient<T> { &self.a }

  /// Function gets `B` coefficient from certain EC variant.
  pub fn get_b(&self) -> T { self.b.clone() }

  /// Function gets reference to `B` coefficient from certain EC variant.
  pub fn get_ref_b(&self) -> &T { &self.b }

  /// Function gets base point from certain EC variant.
  /// It has big prime order in EC that is saved in `EC.get_ord()`.
  pub fn get_bp(&self) -> AffinePoint<T> { self.bp.clone() }

  /// Function gets reference to base point from certain EC variant.
  /// It has big prime order in EC that is saved in `EC.get_ord()`.
  pub fn get_ref_bp(&self) -> &AffinePoint<T> { &self.bp }

  /// Function gets big prime order of base point.
  pub fn get_ord(&self) -> BigUint { self.n.clone() }

  /// Function gets reference to the big prime order of base point.
  pub fn get_ref_ord(&self) -> &BigUint { &self.n }

  /// Function checks whether point belongs to the specified curve
  /// by calculating this equation: $y^2 + xy = x^3 + Ax^2 + B$.
  pub fn check_affine_point(&self, point : &AffinePoint<T>) -> bool
  {
    match point
    {
      AffinePoint::Point { x: x_p, y: y_p } =>
      {
        y_p.square() + x_p.clone() * y_p.clone() == x_p.clone().pow(3_u8) + (self.a.as_field_el() * x_p.square()) + self.b.clone()
      }
      AffinePoint::Infinity => false,
    }
  }

  /// Function performs _packing_ of point that has odd prime order in EC over GF(2^m)
  /// according to the algorithm `6.9`.
  /// Overrides function from [AffinePoint::unpack].
  pub fn unpack_affine_point(&self, num : &T) -> AffinePoint<T> { unpack_affine_point(num, self) }

  /// Function performs _unpacking_ of point that has odd prime order in EC over GF(2^m)
  /// according to the algorithm `6.10`.
  /// Overrides function from [AffinePoint::pack].
  pub fn pack_affine_point(&self, point : &AffinePoint<T>) -> T { pack_affine_point(point) }

  /// Function performs addition in affine coordinates.
  /// Related to function [AffinePoint::add].
  pub fn add(&self, p : &AffinePoint<T>, q : &AffinePoint<T>) -> AffinePoint<T> { p.add(self, q) }

  /// Function performs doubling of point in affine coordinates.
  /// Related to function [AffinePoint::double].
  pub fn double(&self, p : &AffinePoint<T>) -> AffinePoint<T> { p.double(self) }

  /// Function performs multiplication on number in affine coordinates.
  /// Related to function [AffinePoint::mul].
  pub fn mul<N : Into<BigUint>>(&self, point : &AffinePoint<T>, n : N) -> AffinePoint<T> { point.mul(self, n) }
}
