# x86 coverage roadmap for libmwemu

## Progress

**COMPLETE — NOEXEC 455 → 3.** Every deterministic x86-64 instruction in the
corpus is now implemented and verified at 0 fail (300 rows/mnemonic), all 708
mnemonics. The only three that stay `not-executed` are `cpuid`, `rdpid` and
`rdpkru`, whose result is the *physical CPU's* feature/ID/PKRU state and so can
never match a fixed oracle (non-matchable by design).

This covers, on top of the earlier non-AVX work, the **entire VEX AVX/AVX2
surface**: vertical integer, packed & scalar float, shifts (fixed and variable
vpsllv/vpsrav/vpsrlv), shuffles/permutes (vpshuf*, vshuf*, vperm2f128/i128,
vpermd/q/ps/pd, vpermilps/pd), unpacks/packs, blends, broadcasts, extract/insert
(vextract/vinsert f128/i128, vpextr*/vpinsr*), conversions (vcvt* including the
lane-changing widen/narrow and **f16 vcvtph2ps/vcvtps2ph, bit-exact**), compares
(vcmp*/vcomi/vucomi/vtest/vptest), crypto (VAES*, VPCLMULQDQ, GFNI), the full
**FMA** family, and **VNNI** (vpdpbusd/wssd). A root-cause fix landed in the
harness for mwemu's separate xmm/ymm register storage (mirror low-128 on set),
which cleared the mixed `ymm,xmm` forms.

`cargo test` stays green (345 pass). Remaining known non-bit-exact cases stay as
documented: rcp/rsqrt (HW ~12-bit approximation), the float NaN-sign corner
(vfnmadd/vfnmsub/scalar), and mpsadbw high-offset windows.

Generic `avx::{binop,unop,scalar,lanes,fps,fpd,shift,broadcast,binop_imm,
unop_imm,fma_*,ternop_acc,pmovx,f16_to_f32,f32_to_f16}` helpers keep almost every
AVX handler to ~5 lines.

---

### Historical

**NOEXEC 455 → ~118.** All deterministic non-AVX plus most of the VEX AVX/AVX2
are implemented (~335 handlers, verified at 0 fail except the documented float
NaN corner). Beyond the earlier non-AVX work, AVX now covers: vertical integer
(add/sub/mul/cmp/min/max/avg/abs/sign/madd/sadbw across byte…qword), packed &
scalar float (vadd/sub/mul/div/sqrt/max/min/cmp/round + vaddsub/vhadd), shifts
(vpsll/srl/sra), shuffles/unpacks/packs (vpshufd/hw/lw, vshufps/pd, vpunpck*,
vpack*, vpshufb), blends (vblend*, vpblendw), broadcasts (vbroadcast*/
vpbroadcast*), VEX crypto (VAES*, VPCLMULQDQ), vcomi/vucomi/vptest, and the full
**FMA** family (vfmadd/vfmsub/vfnmadd/vfnmsub/vfmaddsub/vfmsubadd × 132/213/231 ×
ps/pd/ss/sd) via `avx::fma_*`. Generic `avx::{binop,unop,scalar,lanes,fps,fpd,
shift,broadcast,binop_imm,unop_imm,fma_*}` helpers keep each handler ~5 lines.

**Remaining (~118):** conversions (vcvt*, lane-count-changing), cross-lane
permutes (vperm*, vextract/vinsert f128/i128), gathers, vpmovsx/zx (widening),
vmaskmov; and the bulk — **AVX-512** (EVEX, `zmm`, mask registers `k*`), which
needs that register state added to mwemu first. Also: the `ymm,xmm` mixed forms
(e.g. vpbroadcast from xmm) surface a mwemu xmm/ymm register-aliasing gap worth a
separate fix. Non-matchable by design stay as-is: cpuid/rdpid/rdpkru, rcp/rsqrt
(HW approximation), and the float NaN-sign corner (vfnmadd/vfnmsub/scalar float).

**Earlier — NOEXEC 455 → ~220.** Large slice of AVX (~230 handlers). AVX so far: the
generic `avx::{binop,unop,scalar32,scalar64,lanes,fps,fpd}` helpers drive the
VEX vertical/scalar ops — vp add/sub/mul/cmp/min/max/avg/abs across byte/word/
dword/qword, the float v-forms (vaddps/pd/ss/sd … vsqrt, vaddsub, vunpck),
vpack*, and the VEX crypto (VAES*, VPCLMULQDQ) and vcomi/vucomi. `E2` (ymm in the
harness) is done, so 256-bit forms are tested too.

**Remaining AVX (~220):** shifts (vpsll/srl/sra + by-imm), shuffles/permutes
(vpshufb/d/hw/lw, vperm*, vshufps), broadcasts (vbroadcast*, vpbroadcast*),
inserts/extracts (vinsert*/vextract*), conversions (vcvt*), gathers, FMA, and the
full AVX-512 (EVEX, `zmm`, mask registers `k*`) — the last needs new register
state before it can be modelled.

**Earlier milestone — NOEXEC 455 → 302** (deterministic non-AVX), ~155 handlers:
BMI1, all of SSSE3, SSE4.1 (integer + float: blends, round, dp, extract/insert),
SSE4.2 (crc32, pcmpestri/pcmpestrm, pcmpgtq), the SSE/SSE2/SSE3 float math
(div/max/min/sqrt/cmp/cvt/addsub/hadd/hsub/unpck/shuf/dup/movmsk), adcx/adox,
sahf, and the full crypto set (AES-NI, PCLMULQDQ, GFNI, SHA-1/SHA-256).
`E1` (undefined-flag masking via iced-x86) is done, so the scorecard is clean.
The `cargo test` suite stays green (345 pass).

**Remaining:** the 297 AVX/AVX2/AVX-512 (`v*`) mnemonics (Phase 3, needs `E2`
ymm harness support), plus a few instructions whose result is inherently
CPU/environment-specific and cannot match a fixed oracle (`cpuid`, `rdpid`,
`rdpkru`) or is a documented hardware *approximation* (`rcpps`/`rcpss`/
`rsqrtps`/`rsqrtss`) or is only partially modelled (`mpsadbw` high-offset windows).

---


Derived from `mwemu-x86-test` against the exhaustive x86Tester corpus (708
mnemonics generated on an Intel Core Ultra 7 265U). Of those: ~208 are executed
and testable, **455 have no engine handler** (NOEXEC), and 45 cannot be tested
yet because the harness does not model their state (x87 `st(i)`, `ymm`).

Breakdown of the 455 missing:

| Family | Count | Notes |
|---|---|---|
| AVX / AVX2 / AVX-512 (VEX/EVEX `v*`) | 297 | even VEX.128 forms of implemented SSE ops |
| SSE/SSE2/SSE3 float vector math | ~78 | add/sub/mul/div/max/min/sqrt/cmp/cvt for ps/pd/ss/sd |
| SSE4.1 integer / blend / round | ~35 | pmovsx/pmovzx, pmulld, round*, blend*, ptest… |
| crypto (AES-NI / SHA / PCLMUL / GFNI) | 17 | aes*, sha*, pclmulqdq, gf2p8* |
| SSSE3 | ~15 | palignr, pabs*, phadd*/phsub*, psign*, pmaddubsw… |
| BMI1 | 6 | andn, bextr, blsi, blsr, pext, rorx |
| SSE4.2 / misc int | 7 | pcmpestri/m, pcmpgtq, crc32, adcx, adox, cpuid |

Priorities are for mwemu's mission (emulating real binaries / malware): favour
what modern compilers and packers actually emit.

---

## Cross-cutting enablers (do these first — they unblock measurement)

- **E1 — undefined-flag masking in the harness** (`iced-x86` `rflags_undefined()`).
  Removes the flag noise so real mismatches stand out. Small, high leverage.
- **E2 — `ymm` read/write in the harness driver.** Currently AVX 256-bit forms
  land in "unsupported" and cannot be scored. Required before Phase 3 can be
  measured, and unblocks the 12 AVX-lane ops now stuck in UNSUP.
- **E3 — x87 `st(i)` / `x87status` modeling in the harness.** Required to score
  the ~33 x87 mnemonics (Phase 5).
- **E4 — harness speed:** reuse one `Emu` reset per row instead of `emu64()` per
  row, so the full ~16M-row corpus runs in seconds, not minutes.

## Phase 1 — Scalar & SSE integer gaps (high impact, low effort, no new state)

Single instructions over existing GPR/XMM state; each is a small lane loop or a
direct computation. These appear constantly in optimized libc, memcpy/string
routines, and compiler output.

- **BMI1:** `andn`, `bextr`, `blsi`, `blsr`, `pext`, `rorx`
- **`cpuid`** — anti-analysis fingerprinting; frequent in malware.
- **`adcx` / `adox`** — big-integer add (crypto/bignum).
- **`crc32`** (SSE4.2) — hashing.
- **SSSE3:** `palignr`, `pabsb/w/d`, `phaddw/d`, `phsubw/d`, `phaddsw/phsubsw`,
  `pmaddubsw`, `pmulhrsw`, `psignb/w/d`
- **SSE4.1 integer:** `pmovsx{bw,bd,bq,wd,wq,dq}`, `pmovzx{…}` (sign/zero extend),
  `pmulld`, `pminsd/pmaxsd/pminud/pmaxud`, `pcmpeqq`, `packusdw`, `ptest`,
  `phminposuw`, `pblendvb/pblendw`, `insertps/extractps`
- **SSE4.2 string:** `pcmpestri`, `pcmpestrm`, `pcmpgtq` (optimized strlen/strchr)

## Phase 2 — SSE/SSE2/SSE3 float vector math (medium impact, medium effort)

~78 ops: `add/sub/mul/div/max/min {ps,pd,ss,sd}`, `sqrt*`, `rcp*`, `rsqrt*`,
`cmpps/pd/ss/sd`, the `cvt*` conversion family, `haddps/pd`, `hsubps/pd`,
`addsubps/pd`, `movsldup/movshdup/movddup`, `round{ps,pd,ss,sd}`, `dpps/dppd`.
Fold in the **known NaN/special-value corner** here (see `BUGS_x86.md`): match
the silicon's QNaN propagation and `NaN − NaN`.

## Phase 3 — AVX / AVX2 (VEX) — biggest coverage win, largest effort

297 mnemonics. Needs (a) YMM state wired through the engine, (b) the VEX
handlers. Most VEX.128 forms mirror an existing SSE handler on the low 128 bits
while **zeroing [255:128]**; VEX.256/AVX2 extend to two lanes. Suggested split:

- **3a — AVX.128:** reuse the (now-fixed) SSE lane logic, add the upper-zeroing.
- **3b — AVX.256 / AVX2:** two-lane forms, `vperm*`, `vextract*/vinsert*`,
  gathers deferred.

Depends on E2 (harness ymm) to be measurable.

## Phase 4 — Crypto acceleration (targeted, well-specified)

17 mnemonics, common in ransomware/packers: `aesenc/aesdec/aesenclast/
aesdeclast/aesimc/aeskeygenassist`, `sha1{msg1,msg2,nexte,rnds4}`,
`sha256{msg1,msg2,rnds2}`, `pclmulqdq`, `gf2p8{affineqb,affineinvqb,mulb}`.
Each is a fixed algorithm; implement against known test vectors.

## Phase 5 — x87 FPU (legacy, needs E3)

~33 mnemonics (`fadd/fmul/fdiv/fsqrt/fsin/fcos/fcom/fxch/fnstsw/fninit/fprem/
frndint/fscale/fcmov*`…). Still present in 32-bit malware. Blocked on E3 (harness
x87 modeling) before it can be scored.

## Deprioritized — AVX-512 (EVEX / `zmm`)

Large surface, rare in the malware corpus, needs `zmm`/mask-register state.
Revisit after Phases 1–4.

---

### Suggested order

`E1 + E4` (denoise + speed) → **Phase 1** (fast, broad real-world coverage) →
`E2` → **Phase 3a** (AVX.128, the single biggest gap) → **Phase 2** (float) →
**Phase 4** (crypto) → **Phase 3b** (AVX.256/AVX2) → `E3` + **Phase 5** (x87).
