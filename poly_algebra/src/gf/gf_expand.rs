#[macro_export]
macro_rules! impl_gf_for_poly {
  ($tn:ty, $p_poly_slice:expr, $p_poly_lazy_cell:expr) => {
    impl GFFactory<'_> for $tn
    {
      #[inline]
      fn new(mut poly : BigUint, prime_poly : BigUint, _ : SealingStruct) -> Self
      {
        module_reduction(&mut poly, &prime_poly);
        Self { poly, prime_poly }
      }

      #[inline]
      fn create_prime_poly() -> BigUint { $p_poly_lazy_cell.clone() }
    }

    impl GFGetters for $tn
    {
      #[inline]
      fn get_prime_poly(&self) -> BigUint { self.prime_poly.clone() }

      #[inline]
      fn get_value(&self) -> BigUint { self.poly.clone() }

      #[inline]
      fn get_ref_prime_poly(&self) -> &BigUint { &self.prime_poly }

      #[inline]
      fn get_ref_value(&self) -> &BigUint { &self.poly }
    }

    impl<'a> GFArithmetic<'a> for $tn
    {
      fn rand(rng : &mut impl CryptoRngCore) -> Self
      {
        <Self as GFArithmetic>::from_poly($crate::helpers::generate_num(rng, Self::get_m()))
      }

      fn get_m() -> u32 { $p_poly_slice[0] }
    }

    impl One for $tn
    {
      fn one() -> Self
      {
        Self {
          poly : BigUint::one(),
          prime_poly : <$tn as GFFactory>::create_prime_poly(),
        }
      }
    }

    impl Zero for $tn
    {
      fn zero() -> Self
      {
        Self {
          poly : BigUint::zero(),
          prime_poly : <$tn as GFFactory>::create_prime_poly(),
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

      fn mul(self, rhs : Self) -> Self::Output
      {
        let mut num = self.clone();
        mul(&mut num.poly, &rhs.poly, &num.prime_poly);
        num
      }
    }

    impl Add<Self> for &$tn
    {
      type Output = $tn;

      fn add(self, rhs : Self) -> Self::Output
      {
        let mut num = self.clone();
        add(&mut num.poly, &rhs.poly);
        num
      }
    }
    impl Mul<&Self> for &$tn
    {
      type Output = $tn;

      fn mul(self, rhs : &Self) -> Self::Output
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

      fn mul(self, rhs : BigUint) -> Self::Output
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

      fn add(self, rhs : BigUint) -> Self::Output
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

      fn mul(self, rhs : &BigUint) -> Self::Output
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

    impl From<$tn> for BigUint
    {
      fn from(value: $tn) -> Self {
        value.poly
      }
    }
  };
}
