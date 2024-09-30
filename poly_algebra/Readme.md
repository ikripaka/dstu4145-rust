## PolyAlgebra 

Binary field integer arithmetic with usage of `BigUint` from `num-bigint`.
Provides specific implementation of binary Galois Fields for `DSTU 4145-2002` standard.
Common numerical operations for Galois fields are overloaded, so we can treat them the same way we treat other numbers.

## Overview

This module is dedicated to the usage with the DSTU 4145-2002 signing algorithm that is using arithmetic over binary field. Needless to note that standart you can find by this [link](https://www.ksv.biz.ua/GOST/DSTY_ALL/DSTU2/dstu_4145-2002.pdf) or internally in [repository](../assets/dstu_4145-2002.pdf) for archiving purposes.

There are types that are implemented as unsigned integers in Galois Binary Field (GF 2^m) with usage of the [recommended](https://zakon.rada.gov.ua/laws/show/z1399-12#n4) by Ukrainian government prime polynomials.
* [`GF163`] - GF 2^163 over prime polynomial `x^163 + x^7 + x^6 + x^3 + 1`.
* [`GF167`] - GF 2^167 over prime polynomial `x^167 + x^6 + 1`.
* [`GF173`] - GF 2^173 over prime polynomial `x^173 + x^10 + x^2 + x + 1`.
* [`GF179`] - GF 2^179 over prime polynomial `x^179 + x^4 + x^2 + x + 1`.
* [`GF191`] - GF 2^191 over prime polynomial `x^191 + x^9 + 1`.
* [`GF233`] - GF 2^233 over prime polynomial `x^233 + x^9 + x^4 + x^1 + 1`.
* [`GF257`] - GF 2^257 over prime polynomial `x^257 + x^12 + 1`.
* [`GF307`] - GF 2^307 over prime polynomial `x^307 + x^8 + x^4 + x^2 + 1`.
* [`GF367`] - GF 2^367 over prime polynomial `x^367 + x^21 + 1`.
* [`GF431`] - GF 2^431 over prime polynomial `x^431 + x^5 + x^3 + x + 1`.

## Example 
```rust

use num_bigint::ParseBigIntError;
use num_traits::One;
use poly_algebra::gf::{GFArithmetic, GFDisplay, GF257};
#fn main() -> Result<(), ParseBigIntError>
#{
  let a = GF257::from_hex_be("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551")?;
  let b = a.inverse();
  assert_eq!(
    (a.clone() + &b).to_lower_hex_be(),
    "15dcae3326c3a36c142df9ade7c5db382056cfcdad7bd63e5b558cc16ffc0f09e"
  );
  assert_eq!(a * b, GF257::one());
#  Ok(())
#}

```

## Importing

* From git: 
```toml
[dependencies]
poly_algebra = { git = "https://github.com/ikripaka/dstu4145-rust/poly-algebra"}
num-traits = "0.2"
num-bigint = "0.4.6"
```

* From local:
```toml
[dependencies]
poly_algebra = { path = "<path_to_folder>/dstu4145-rust/poly_algebra"}
num-traits = "0.2"
num-bigint = "0.4.6"
```