# Result for now

## hexagon-sim 문제들

- `if ([!]Pu.new)` instruction들의 경우 새 `Pu`값의 least significant bit (& 0x01) 값을 true/false의 기준으로 삼는 것을 확인할 수 있었음.
- `sfcmp.uo`, `dfcmp.uo` instruction의 경우 True를 반환하게 하는 input case에 대해서 USR의 invalid sticky flag를 raise함.

## quic/qemu 문제들

- `Rd=extractu(Rs,#u5,#U5)` instruction의 경우 `(Rs >> #U5) & ((1 << #u5) - 1)` 값을 반환해야 하는데, quic/qemu instruction을 보면 `0 < #U5 + #u5 <= 32`를 가정하고 `(Rs >> (32 - #U5 - #u5)) << (32 - #u5)`로 작성되어 있음. (`tcg_gen_extract_i32` 참조) 그러나 instruction 설명의 어디에도 해당 조건을 만족해야 한다는 문장은 없음.
- `Rxx,Pe=vacsh(Rss,Rtt)` instruction의 계산 과정에서 RxxV를 잘못 재활용하는 것으로 보임.
```
#define fGEN_TCG_A5_ACS(SHORTCODE) \
    do { \
        gen_helper_vacsh_val(RxxV, cpu_env, RxxV, RssV, RttV); \
        gen_helper_vacsh_pred(PeV, cpu_env, RxxV, RssV, RttV); \
    } while (0)
```
`PeV`의 계산 과정에서 과거의 `RxxV` 값이 사용되어야 하지만 그렇지 않아서 predicate register 값이 정상적으로 계산되지 않음.

## 양쪽 문제들

- `Rd=convert_df2uw(Rss):chop` instruction의 경우 다음과 같이 정의되어 있음.

```
Convert floating point values. If rounding is required, it happens according to the rounding
mode unless the :chop option is specified. If the value is out of range of the destination
integer type, the INVALID flag is raised and closest integer is chosen, including for
infinite inputs. For NaN inputs, the INVALID flag is also raised, and the output value is
IMPLEMENTATION DEFINED.
```

hexagon-sim의 경우, value가 정상적으로 destination type의 range 안에 있음에도 INVALID flag가 raise되는 것을 확인할 수 있었음. 반면, quic/qemu의 경우 INEXACT flag를 종종 raise하는 것을 확인할 수 있었음.

NaN에 대해서는 hexagon-sim은 0xffffffff, quic/qemu는 0x00000000을 반환하나, 이는 문서에 따라 IMPLEMENTATION DEFINED이므로 문제가 없음.

## 확인 안 된 문제들

- sfrecipa instruiction의 predicate register output이 양쪽에서 다르게 계산되며, 이 부분은 spec에 따라서는 문제가 없는 것으로 보임.

```
Provides an approximation of the reciprocal of the denominator (Rt), if combined with the
appropriate fixup instructions. Certain values (such as infinities or zeros) in the numerator
or denominator may yield values that are not reciprocal approximations, but yield the
correct answer when combined with fixup instructions and the appropriate routines.
For compatibility, do not rely on exact results of these instructions. The precision of the
approximation for this architecture and later is at least 6.6 bits.
```

```
if ((Rs,Rt,Rd,adjust)=recip_common(Rs,Rt)) {
    Pe = adjust;
    idx = (Rt >> 16) & 0x7f;
    mant = (recip_lut[idx] << 15) | 1;
    exp = 127 - (exponent(Rt) - 127) - 1;
    Rd = -1**Rt.31 * 1.MANT * 2**(exp-BIAS);
}
```