# DSTU-4145 signature

Crate is designed specifically as a capstone project to Ukrainian Rust community [bootcamp](https://github.com/rust-lang-ua/rustcamp).
It provides DSTU-4145 implementation in Rust, but with some nuances: 
* GOST hashing function is changed to SHA3 one with equal resulted hash lengths;
* there are no support to normal basis algorithms as (as far I searched) Rust don't have any implementation of it.


To sum up signing algorithm we can say that it corresponds to the following steps:
1) Check of the income parameters.
2) Check of private key.
3) L_d - 
4) Check of the hash identifier.
5) Check hash length violation.
6) Calculation of H(T)
7) Mapping of H(T) as the element of the prime field used for numbers calculation.
8) Calculation of presign F_e and corresponding parameter e.
9) $y = h * F_e$, where y - field element
10) Transforming y on number r. If r = r, repeat from 8)
12) Calculating $s=(e+dr) mod n$, s=0 - repeat from 8)
14) Pair (r, s) - digital sign D -> (iH, T, D), where L_D >= 2L(n)

Check algorighm:








Interesting fact that it is available [scheme for truncating](https://ela.kpi.ua/handle/123456789/62109) DSTU-4145(in original implementation) signatures regarding to the same work of [Tomas Pornin](https://eprint.iacr.org/2022/938). 
