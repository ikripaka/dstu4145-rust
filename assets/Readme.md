#### DSTU 4145-2002 parameters
* $m$ - degree of prime field $F_2$ that is chosen from standard tables;
* $P$ = $(x_p, y_p)$ - base point in EC;
* $d$ - private key (calculated with help of PRNG);
* $Q$ = $-dP$;
* $n$ - order of base point in EC, odd prime;
* $T$ - stands for plaintext with bit length $L_T$ > 0;
* $L_D$ - bit length of signature;
* $L_h$ - bit length of hashing algorithm output;
* $L(n)$ - bit length of EC order; 
* $iH$ - hash function identifier;
* $H$ - `GOST 34.311` hash function or any other hash function, recommended by an authorized state authority in the field cryptographic protection of information (by default in our implementation is `SHA3-512`, but you can use your own with `try_sign_digest()`).

#### Private key
Number $d$ with length $L(n) - 1$;
#### Public key
Point on EC - $Q$, where $Q = -dP$.

#### Signing 
##### Input
* general parameters;
* plaintext message $T$ with length $L_T > 0$;
* hashing function $H$;
* length of the signature $L_D$.
##### Steps
To sum up signing algorithm we can say that it corresponds to the following steps:
1) Check of the income parameters.
2) Check of private key validity (a.k.a. check point order).
3) Check $L_d$ value. $L_d | 16$, $>= 2L(n)$.
4) Check of the hash identifier.
5) Check hash length violation (maybe we have some regulating policies).
6) Calculation of $H(T)$.
7) Mapping of $H(T)$ as the element of the prime field used for numbers calculation (truncating hash to '$L(n) - 1$' bit size).
8) Calculation of presign F_e and corresponding parameter e.
   * Generating of random number $e$, $0 < e < n$.
   * Calculating $eP$, where is base point.
   * if $x_p$ is 0, repeat one more time from 8).
   * Getting $x_p$ coordinate from previous calculated point.
9) $y = h * F_e$, where y - field element ($y \in GF(2^m)$)
10) Transforming $y$ on number $r$. If $r = r$, repeat from 8)
12) Calculating $s=(e+dr) mod n$, $s=0$ - repeat from 8)
14) Pair $(r, s)$ - digital sign $D \rightarrow (iH, T, D)$, where $L_D >= 2L(n)$

#### Verifying
##### Input
* general parameters;
* public key $Q$;
* signed message $(iH, T, D)$;
* hashing function $H$.
##### Steps
To sum up verifying algorithm we can say that it corresponds to the following steps:
1) Check of $iH$ (to conform regulating policies).
2) Determine $L_h$ from $iH$.
3) Check $L_d >= 2L(n)$, $L_d | 16$.
4) Check of the parameters validity (checking correctness of binary field, EC parameters).
5) Check of the public key correctness due to algorithms (check belonging of the $Q$ to the EC).
6) Check $L_T$ correctness (that $L_T = L(iH) + L_T + L_D$). 
7) $T \rightarrow H(T)$.
8) Transform $H(T)$ into field element $h$.
9) Destructure signature into components, $D \rightarrow (r,s)$.
10) Check $r$, $0 < r < n$.
11) Check $s$, $0 < s < n$.
12) Calculate $R = sP + rQ$, $R = (x_r, y_r)$.
13) Calculate in field $y = h*x_r$, $y \in GF(2^m)$.
14) Transform field element in ordinary number $r^{'}$ (truncate number to $L(n)$ bits).
15) Check $r == r^{'}$


#### Application of this certain digital signature?
* This type of digital signature is applied to the most of the providers of electronic trust services, customers, developers in Ukraine.
* Truncating digital signature, as referred to this work: [link](https://ela.kpi.ua/items/996b943b-6a28-4a89-a351-88849b0318fc). Mainly for using in archiving purposes.
* Moreover, you can find more interesting works on it by typing `ДСТУ 4145-2002` in any search engine or, for example in student works repository in KPI, [link](https://ela.kpi.ua/home).