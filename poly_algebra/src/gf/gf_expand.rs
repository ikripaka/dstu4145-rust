#[macro_export]
macro_rules! impl_gf_for_poly {
  ($tn:ty, $p_poly:expr) => {
    impl GFFactory<'_> for $tn
    {
      fn new(poly : BigUint, prime_poly : BigUint, _ : SealingStruct) -> Self { Self { poly, prime_poly } }

      fn create_prime_poly() -> BigUint { create_prime_polynomial($p_poly) }
    }

    impl GFGetters for $tn
    {
      fn get_prime_poly(&self) -> BigUint { self.prime_poly.clone() }

      fn get_value(&self) -> BigUint { self.poly.clone() }
    }

    impl<'a> GFArithmetic<'a> for $tn
    {
      fn rand(rng : &mut impl CryptoRngCore) -> Self
      {
        <Self as GFArithmetic>::from_poly(crate::helpers::generate_num(rng, Self::get_m()))
      }

      fn get_m() -> u32 { $p_poly[0] }
    }

    impl One for $tn
    {
      fn one() -> Self
      {
        Self {
          poly : BigUint::one(),
          prime_poly : <GF3 as GFFactory>::create_prime_poly(),
        }
      }
    }

    impl Zero for $tn
    {
      fn zero() -> Self
      {
        Self {
          poly : BigUint::zero(),
          prime_poly : <GF3 as GFFactory>::create_prime_poly(),
        }
      }

      fn is_zero(&self) -> bool { self.poly.is_zero() }
    }

    impl Mul<Self> for $tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : Self) -> Self::Output
      {
        mul(&mut self.poly, &rhs.poly, &self.prime_poly);
        self
      }
    }

    impl Add<Self> for $tn
    {
      type Output = $tn;

      fn add(mut self, rhs : Self) -> Self::Output
      {
        add(&mut self.poly, &rhs.poly);
        self
      }
    }

    impl Mul<&Self> for $tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : &Self) -> Self::Output
      {
        mul(&mut self.poly, &rhs.poly, &self.prime_poly);
        self
      }
    }

    impl Add<&Self> for $tn
    {
      type Output = $tn;

      fn add(mut self, rhs : &Self) -> Self::Output
      {
        add(&mut self.poly, &rhs.poly);
        self
      }
    }

    impl Mul<Self> for &$tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : Self) -> Self::Output
      {
        let mut num = self.clone();
        mul(&mut num.poly, &rhs.poly, &num.prime_poly);
        num
      }
    }

    impl Add<Self> for &$tn
    {
      type Output = $tn;

      fn add(mut self, rhs : Self) -> Self::Output
      {
        let mut num = self.clone();
        add(&mut num.poly, &rhs.poly);
        num
      }
    }
    impl Mul<&Self> for &$tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : &Self) -> Self::Output
      {
        let mut num = self.clone();
        mul(&mut num.poly, &rhs.poly, &num.prime_poly);
        num
      }
    }

    impl Add<&Self> for &$tn
    {
      type Output = $tn;

      fn add(self, rhs : &Self) -> Self::Output
      {
        let mut num = self.clone();
        add(&mut num.poly, &rhs.poly);
        num
      }
    }

    ///
    impl Mul<BigUint> for $tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs);
        mul(&mut self.poly, &rhs.poly, &self.prime_poly);
        self
      }
    }

    impl Add<BigUint> for $tn
    {
      type Output = $tn;

      fn add(mut self, rhs : BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs);
        add(&mut self.poly, &rhs.poly);
        self
      }
    }

    impl Mul<&BigUint> for $tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : &BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs.clone());
        mul(&mut self.poly, &rhs.poly, &self.prime_poly);
        self
      }
    }

    impl Add<&BigUint> for $tn
    {
      type Output = $tn;

      fn add(mut self, rhs : &BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs.clone());
        add(&mut self.poly, &rhs.poly);
        self
      }
    }

    impl Mul<BigUint> for &$tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs);
        let mut num = self.clone();
        mul(&mut num.poly, &rhs.poly, &num.prime_poly);
        num
      }
    }

    impl Add<BigUint> for &$tn
    {
      type Output = $tn;

      fn add(mut self, rhs : BigUint) -> Self::Output
      {
        let mut num = self.clone();
        let rhs = <$tn as GFArithmetic>::from_poly(rhs);
        add(&mut num.poly, &rhs.poly);
        num
      }
    }
    impl Mul<&BigUint> for &$tn
    {
      type Output = $tn;

      fn mul(mut self, rhs : &BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs.clone());
        let mut num = self.clone();
        mul(&mut num.poly, &rhs.poly, &num.prime_poly);
        num
      }
    }

    impl Add<&BigUint> for &$tn
    {
      type Output = $tn;

      fn add(self, rhs : &BigUint) -> Self::Output
      {
        let rhs = <$tn as GFArithmetic>::from_poly(rhs.clone());
        let mut num = self.clone();
        add(&mut num.poly, &rhs.poly);
        num
      }
    }
  };
}

#[macro_export]
macro_rules! impl_gf_display {
  ($tn:ty) => {
    impl GFDisplay for $tn {}

    impl fmt::Debug for $tn
    {
      fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "{}", format!("{:X?}", self.poly)) }
    }

    impl fmt::Display for $tn
    {
      fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "{}", format!("{:X?}", self.poly)) }
    }

    impl fmt::LowerHex for $tn
    {
      fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "{}", to_lower_hex_be(&self.poly)) }
    }

    impl fmt::UpperHex for $tn
    {
      fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "{}", to_upper_hex_be(&self.poly)) }
    }

    impl fmt::Binary for $tn
    {
      fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "{}", to_binary_be(&self.poly)) }
    }
  };
}

#[macro_export]
macro_rules! impl_gf_conversions {
  ($tn:ty) => {
    impl From<BigUint> for $tn
    {
      fn from(mut value : BigUint) -> Self
      {
        let mut num = <$tn as num_traits::Zero>::zero();
        module_reduction(&mut value, &num.prime_poly);
        let _ = mem::replace(&mut num.poly, value);
        num
      }
    }

    impl Into<BigUint> for $tn
    {
      fn into(self) -> BigUint { self.poly }
    }
  };
}

#[macro_export]
macro_rules! impl_obj_safe_gf_for_poly {
  ($tn:ty, $p_poly:expr) => {
    impl<'a> GFFactoryObjSafe<'a> for $tn
    {
      fn new(poly : BigUint, prime_poly : BigUint, _ : SealingStruct) -> Box<Self> { Box::new(Self { poly, prime_poly }) }

      fn create_prime_poly() -> BigUint { create_prime_polynomial(&$p_poly) }
    }

    impl<'a> GFArithmeticObjSafe<'a> for $tn
    {
      fn rand(rng : &mut impl CryptoRngCore) -> Box<Self>
      {
        <Self as GFArithmeticObjSafe>::new(crate::helpers::generate_num(rng, $p_poly[0]))
      }

      fn one() -> Box<Self> { Box::new(<Self as num_traits::One>::one()) }

      fn is_one(&self) -> bool { <Self as num_traits::One>::is_one(self) }

      fn zero() -> Box<Self> { Box::new(<Self as num_traits::Zero>::zero()) }

      fn is_zero(&self) -> bool { <Self as num_traits::Zero>::is_zero(self) }

      fn add(self, other : &Self) -> Box<Self> { Box::new(self + other) }

      fn mul(self, other : &Self) -> Box<Self> { Box::new(self * other) }
    }
  };
}
