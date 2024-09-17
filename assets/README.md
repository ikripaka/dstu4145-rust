#### DSTU 4145-2002 parameters
* m - degree of prime field F_2 that is chosen from standart tables;
* P = (x_p, y_p) - base point in EC;
* d - private key (calculated with help of PRNG);
* Q = -dP;
* n - order of base point in EC, odd prime;
* T - stands for plaintext with length L_T > 0;
* L_D - length of signature;
* H - GOST 34.311 hash function or any other hash function, recommended by an authorized state authority in the field cryptographic protection of information (in our implementation is SHA3-256).

#### Signing 
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

