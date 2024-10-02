## dstu4145-rust

DSTU 4145-2002 rust implementation. Provides interface for signing and verifying signatures with help of `signature` crate. Common logic of packing signature & points are implemented.

## Overview

This module implements signing and verification of specific DSTU 4145-2002 signatures. 

_Why specific?_ By default, it uses SHA3-512 signature for signing, but with `sign_digest()` you can use any available digest generating algorithm, even original one [GOST R 34.11-94](https://github.com/RustCrypto/hashes/tree/master/gost94). 

Furthermore, you'll have another question, _do this implementation secure?_
Yes, The parameters were taken from government granted [parameters](https://zakon.rada.gov.ua/laws/show/z1399-12#n4).

Needless to note that, this implementation isn't ideal and by now only suit for academic research usage, because of not so efficient algorithms implemented, especially affine points calculation in EC (has to implement projective coordinates point calculation).

## Example
```rust
# use rand_chacha::ChaCha20Rng;
# use rand_chacha::rand_core::RngCore;
# use signature::rand_core::SeedableRng;
# use signature::{RandomizedSigner, Verifier};
# use dstu4145_rust::sign::{Signature, SigningKey};
# use rust_ec::binary_ec::BinaryEC;

# fn main() -> dstu4145_rust::error::Result<()>
# {
  let mut plaintext = vec![0; 16];
  let mut rng = ChaCha20Rng::from_entropy();
  rng.fill_bytes(&mut plaintext);
  let ec = BinaryEC::generate_m257_pb_curve();
  let (private_key, pub_key) = SigningKey::generate(&mut rng, ec, 1024)?;
  let signature = private_key.sign_with_rng(&mut rng, &plaintext);
  let packed_sign = signature.pack();
  let upacked_sign = Signature::try_from(packed_sign.as_slice())?;
  pub_key.verify(&plaintext, &upacked_sign)?;
  Ok(())
# }
```

## Adding to your project

* From git:
```toml
[dependencies]
dstu4145-rust = {git = "https://github.com/ikripaka/dstu4145-rust/dstu4145-rust"}
rust-ec = { git = "https://github.com/ikripaka/dstu4145-rust/rust-ec"}
poly_algebra = { git = "https://github.com/ikripaka/dstu4145-rust/poly-algebra"}
num-traits = "0.2"
num-bigint = "0.4.6"
```

* From local:
```toml
[dependencies]
dstu4145-rust = {}
rust-ec = { path = "<path_to_folder>/dstu4145-rust/dstu4145-rust"}
poly_algebra = { path = "<path_to_folder>/dstu4145-rust/poly_algebra"}
num-traits = "0.2"
num-bigint = "0.4.6"
```

Implemented:
* Interacting interface with `signature` crate.
* Working standard with usage of arithmetics in polynomial basis.
* Choice between the EC's recommended parameters by Ukrainian government.
* Affine EC points arithmetic.
* Common packing logic.
  Todo:
* Find formulas for Koblitz projective coordinates points calculation.
* Implement projective EC coordinates calculation.
* Implement packing of PK, Parameters into ASN1 files.
* Normal basis calculation.