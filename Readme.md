# DSTU 4145-2002 signature in PB

This crate is designed and developed specifically as a [capstone project][1] to [Ukrainian Rust Community][2] [bootcamp][3].

## Problematic

Previous year when I was preparing my thesis for bachelor's degree with the theme "Truncating El-Gamal type digital
signatures" and have to check my results with [DSTU 4145-2002][4], but there are no available implementation of it on
Rust. So, I have managed somehow to use specific library in C++, and done that successfully :).
Mainly the idea originates from here.

I want to add to what has been said, that when I started implementing it a capstone project, realised, that we don't
have any available library for using algebra over GF2^m, only for prime fields. So, in addition, I have also implemented
that by myself.

## Implementation notes

It provides [DSTU 4145-2002][4] implementation in Rust, but with some nuances:

* GOST hashing function is changed to SHA3-512 one;
* By now there is no support for normal basis arithmetic.

Moreover, with the DSTU implementation, you can use already [Binary Elliptic Curve][7] * over [GF 2^m][8] over some
extension with polynomial basis but for specific polynomials. In addition to that

\* - (p. 19, Koblitz curve with A = 0 or 1)

At this moment I can say that from standard [DSTU 4145-2002][4] is implemented such a list of things:

* Interacting interface with `signature` crate ([dstu4145-rust][11]).
* Working standard with usage of arithmetics in polynomial basis ([poly_algebra][9]).
* Choice between the EC's recommended parameters by Ukrainian government in ([rust-ec][10]).
* Affine EC points arithmetic.
* Common packing logic for signature and public key affine point on EC.

## Quick start

Examples of how to use the project can find in the crates documentation or in [examples folder][6].

## Digital signature scheme

Actual implemented algorithm scheme is represented [here][5]. Necessary details can be found [here][4].

## Future plans

With the passage of time, there are some plans to be done:

* Implement benchmarks.
* Find formulas for Koblitz projective coordinates points calculation.
* Implement projective EC coordinates calculation.
* Normal basis arithmetic calculation.
* ?Implement full packing of Public Key and Parameters into ASN1 structs?.

##### Link to the thesis if you're interested
_Interesting fact that it is available [scheme for truncating][14] DSTU 4145-2002
signatures regarding the same work of [Tomas Pornin][13]._
_And also, [article publication][12] in English (p. 10, 'Kripaka I., Yakovliev S.')._

[1]: https://github.com/rust-lang-ua/rustcamp/tree/master/6_project
[2]: https://github.com/rust-lang-ua
[3]: https://github.com/rust-lang-ua/rustcamp
[4]: https://www.ksv.biz.ua/GOST/DSTY_ALL/DSTU2/dstu_4145-2002.pdf
[5]: ./assets/Readme
[6]: ./examples
[7]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf
[8]: https://en.wikipedia.org/wiki/GF%282%29
[9]: https://github.com/ikripaka/dstu4145-rust/tree/feature/dstu4145-pb/poly_algebra
[10]: https://github.com/ikripaka/dstu4145-rust/tree/feature/dstu4145-pb/rust-ec
[11]: https://github.com/ikripaka/dstu4145-rust/tree/feature/dstu4145-pb/dstu4145-rust
[12]: https://sites.google.com/viti.edu.ua/conference/english?authuser=0&pli=1
[13]: https://eprint.iacr.org/2022/938
[14]: https://ela.kpi.ua/handle/123456789/62109