# rust-ec

Koblitz elliptic curve implementation with usage of binary field arithmetic in it.
Provides specific implementation of Koblitz elliptic curve `DSTU 4145-2002` standard.

## Overview
This module is dedicated to the usage with DSTU 4145-2002. Implementation uses binary field arithmetic from `poly_algebra`. Worth noting that EC you can initialize from all possible implemented types from `poly_algebra`.

## Example
```rust 
#use rand_chacha::ChaCha20Rng;
#use rand_core::SeedableRng;
use poly_algebra::helpers::generate_num;
use rust_ec::{affine_point::AffinePoint, binary_ec::BinaryEC};

#fn main()
#{
#  let mut rng = ChaCha20Rng::from_entropy();
  let ec = BinaryEC::generate_m257_pb_curve();
  let p = ec.get_bp();
  let q = {
    let bp = ec.get_bp();
    let k = generate_num(&mut rng, ec.get_ref_ord().bits()) % ec.get_ref_ord();
    ec.mul(&bp, k)
  };
  let sum = ec.add(&q.negative(), &p);
  let packed_sum = sum.pack();
  let unpack_p = AffinePoint::unpack(&packed_sum, &ec);
  assert_eq!(ec.mul(&unpack_p, ec.get_ord()), AffinePoint::Infinity);
#}
```


## Adding to your project

* From git:
```toml
[dependencies]
rust-ec = { git = "https://github.com/ikripaka/dstu4145-rust/rust-ec"}
num-traits = "0.2"
num-bigint = "0.4.6"
```

* From local:
```toml
[dependencies]
rust-ec = { path = "<path_to_folder>/dstu4145-rust/rust-ec"}
num-traits = "0.2"
num-bigint = "0.4.6"
```