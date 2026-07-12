# x86 semantic bugs found by mwemu-x86-test

Bugs in libmwemu instruction semantics surfaced by replaying hardware-oracle
test vectors (x86Tester, generated on an Intel Core Ultra 7 265U) through the
`mwemu-x86-test` harness and diffing the resulting CPU state against the silicon.

Each was verified by hand against the oracle value and the documented x86
semantics. Reproduce any of them with:

```bash
CORPUS=~/soft/remill-tester/testdata/x86tester-full/Intel_R_Core_TM_Ultra_7_265U_f6m181s0
./target/release/mwemu-x86-test $CORPUS/<mnemonic>.txt --stop-on-first-fail
```

Regression tests for the main ones live in
`crates/libmwemu/src/tests/isa/x64/x86tester_regressions.rs`.

## Fixed implementation bugs

All verified at 300 rows/mnemonic: 300 pass, 0 fail after the fix.

| # | Instruction | Bug | Fix | Status |
|---|---|---|---|---|
| 1 | `movq xmm,xmm` (`F30F7E`) | did not zero bits [127:64] of the destination | mask the moved value to 64 bits | ✅ FIXED |
| 2 | `popcnt` | did not write the status flags (ZF stayed stale) | set ZF=(src==0), clear CF/OF/SF/AF/PF | ✅ FIXED |
| 3 | `ucomiss`/`comiss`/`ucomisd`/`comisd` | did not clear OF/SF/AF; `ucomi*` set only PF on unordered | clear OF/SF/AF; set ZF+PF+CF when unordered | ✅ FIXED |
| 4 | `lzcnt` | ignored operand size (`u64::leading_zeros`), returned 64 for a 16-bit 0 | count at the operand width; set CF/ZF | ✅ FIXED |
| 5 | `cmpxchg reg,reg` | used the full RAX as accumulator and zeroed it; set only ZF | width-correct accumulator (AL/AX/EAX/RAX); full CMP flags | ✅ FIXED |
| 6 | `idiv` (8/16/32/64) | unsigned division | signed division, widened to dodge MIN/-1 overflow, #DE guard | ✅ FIXED |
| 7 | `cvtsi2ss` | zeroed [127:32] and never converted int→float | convert to f32 in [31:0], preserve [127:32] | ✅ FIXED |
| 8a | `paddw` | wrong top-lane mask (26 vs 28 zeros) | per-lane loop with `wrapping_add` | ✅ FIXED |
| 8b | `psubw`/`psubd`/`psubq` | `wrapping_sub` over u128 let a lane borrow flood the upper lanes | mask each lane result to its width | ✅ FIXED |
| 8c | `pcmpgtb` | unsigned byte compare | signed (i8) compare | ✅ FIXED |
| 8d | `psllw` | only 4 of 8 lanes; count read from full u128; wrong count>15 case | per-lane loop, count from low 64 bits | ✅ FIXED |
| 8e | `psrlq` | shifted the whole u128 instead of each 64-bit lane | shift each qword lane independently | ✅ FIXED |
| 8f | `pmaddwd` | only 2 of 4 dwords; could overflow int32 in debug | all dwords, `wrapping_add` of the two products | ✅ FIXED |
| 9 | `xadd reg,reg` (same register) | wrote src after dest, clobbering the sum | skip the src write-back when both operands are the same register | ✅ FIXED |

## Not bugs — undefined behavior

`shld` / `shrd` were initially suspected, but their only divergences are the
**undefined AF flag** and the result in the **count > operand-size region**
(e.g. `shld ax, ax, 0x7F` → count masked to 31, which exceeds 16), both of which
the architecture leaves undefined. No change needed.

## Float NaN / special-value handling (open, low severity)

`addss` `subss` `mulps` `addps` `mulss` … still diverge on ~50% of the swept
inputs — the NaN/inf/denormal rows. mwemu propagates the NaN differently from the
silicon (payload off by one bit, or returns `0` for `NaN − NaN`). x86 does define
QNaN propagation, so these are technically bugs, but they are the FP corner and
normal values pass. Not addressed yet.

## Not bugs — undefined-flag divergence

Fail only on flags the architecture leaves **undefined**, silenced once the
comparator masks per-instruction undefined flags via `iced-x86`'s
`rflags_undefined()`: `imul`/`mul` (SF/ZF/AF/PF), `bsf`/`bsr` (all but ZF),
`sar`/`shr`/`shl` (OF when count ≠ 1), `bzhi`/`blsmsk` (AF/PF),
`rcr`/`rcl`/`rol`/`ror` (OF except count 1).
