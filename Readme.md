# DSTU-4145 signature

Crate is designed specifically as a capstone project to Ukrainian Rust community [bootcamp](https://github.com/rust-lang-ua/rustcamp).
It provides DSTU-4145 implementation in Rust, but with some nuances: 
* GOST hashing function is changed to SHA3 one with equal resulted hash lengths;
* there are no support to normal basis algorithms as (as far I searched) Rust don't have any implementation of it.



Actual implemented algorithm scheme is represented [here](./assets/Readme).








_Interesting fact that it is available [scheme for truncating](https://ela.kpi.ua/handle/123456789/62109) DSTU 4145-2002(in original implementation) signatures regarding the same work of [Tomas Pornin](https://eprint.iacr.org/2022/938)._ 
