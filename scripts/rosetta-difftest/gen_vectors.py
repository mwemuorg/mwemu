"""Record CPU vectors (through Rosetta) into a self-contained Rust test table.

Writes crates/libmwemu/src/tests/isa/x64/cpu_vectors.rs: for every instruction
of the difftest catalogs, one fixed input state and the registers, flags and
memory the real CPU produced. The Rust side (`cpu_vector_tests.rs`) replays each
vector in libmwemu, so CI checks hardware semantics without Rosetta.

    python3 -I gen_vectors.py            # deterministic (seed 2026)
"""
import os, random, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import difftest  # noqa: E402
import difftest_mem  # noqa: E402
from difftest import ALL, CF, PF, AF, ZF, SF, OF, DF, SENTINEL, build_wrappers, rnd64, wrap  # noqa: E402

WINDOW = 128
PER_FAMILY = {"register": 24, "memory": 12}
REG_NAMES = {
    0: "rax eax ax al ah", 1: "rbx ebx bx bl bh", 2: "rcx ecx cx cl ch", 3: "rdx edx dx dl dh",
    4: "rsi esi si sil", 5: "rdi edi di dil", 6: "r8 r8d r8w r8b", 7: "r9 r9d r9w r9b", 8: "r10 r10d r10w r10b",
}
IMPLICIT = {("mul", "imul", "div", "idiv", "cbw", "cwde", "cdqe", "cwd", "cdq", "cqo", "lahf", "sahf", "mulx"): {0, 3},
            ("loop", "loope", "loopne", "jrcxz", "jecxz"): {2},
            ("cmpxchg", "cmpxchg8b", "cmpxchg16b", "xlatb"): {0, 1, 2, 3},
            ("movsb", "movsw", "movsd", "movsq", "stosb", "stosw", "stosd", "stosq", "lodsb", "lodsw", "lodsd", "lodsq",
             "scasb", "scasw", "scasd", "scasq", "cmpsb", "cmpsw", "cmpsd", "cmpsq", "rep", "repe", "repne"): {0, 2, 4, 5}}


def used_regs(text):
    """Indices of the registers an instruction names, plus the implicit ones."""
    words = set(re.findall(r"[a-z0-9]+", text))
    used = {i for i, names in REG_NAMES.items() if words & set(names.split())}
    for mnemonics, regs in IMPLICIT.items():
        if words & set(mnemonics):
            used |= regs
    return used


def trim_unused(regs, text):
    """Zero the registers the instruction does not touch so the vector stays short."""
    keep = used_regs(text)
    return [v if i in keep or i == 9 else 0 for i, v in enumerate(regs)]


def cap(rows, kind):
    by_fam = {}
    for r in rows:
        by_fam.setdefault(r[0], []).append(r)
    out = []
    for fam in sorted(by_fam):
        rs = by_fam[fam]
        out += rs if len(rs) <= PER_FAMILY[kind] else random.sample(rs, PER_FAMILY[kind])
    return out
OUT = os.path.join(HERE, "..", "..", "crates", "libmwemu", "src", "tests", "isa", "x64", "cpu_vectors.rs")


def build_oracles():
    for src in ("oracle.c", "oracle2.c"):
        exe = os.path.join(HERE, src[:-2])
        if not os.path.exists(exe) or os.path.getmtime(exe) < os.path.getmtime(os.path.join(HERE, src)):
            subprocess.run(["clang", "-arch", "x86_64", "-O1", os.path.join(HERE, src), "-o", exe], check=True)


def random_flags(with_df):
    flags = 0
    for b in (CF, PF, AF, ZF, SF, OF) + ((DF,) if with_df else ()):
        if random.random() < 0.5:
            flags |= b
    return flags


def register_vectors(pro, epi):
    cat = difftest.build_catalog(1)
    codes = difftest.asm([e.text for e in cat])
    vectors, meta = [], []
    for e, code in zip(cat, codes):
        regs = [rnd64() for _ in range(9)] + [SENTINEL]
        if e.fix:
            e.fix(regs)
        regs = trim_unused(regs, e.text)
        flags = random_flags(False)
        vectors.append((wrap(code, flags), regs))
        meta.append((e, flags))
    out = difftest.run_oracle(vectors, pro, epi)
    rows = []
    for (code, regs), (e, flags), o in zip(vectors, meta, out):
        if o is None:
            continue
        mask, cmp_regs = e.rule(regs)
        if not cmp_regs:
            continue
        if e.text.startswith("cmpxchg"):
            mask = 0  # Rosetta computes the flags from dest - acc; silicon and libmwemu from acc - dest
        rows.append((e.fam, e.text, code, regs, flags, o, mask, b"", b""))
    return cap(rows, "register")


def memory_vectors(pro, epi):
    difftest_mem.WINDOW = WINDOW
    cat = difftest_mem.build_catalog(1)
    codes = difftest_mem.asm([e.text for e in cat])
    vectors, meta = [], []
    for e, code in zip(cat, codes):
        regs = trim_unused([rnd64() for _ in range(9)] + [SENTINEL], e.text)
        difftest_mem.pin(regs)
        mem = bytearray(difftest_mem.MEMSZ)
        mem[:WINDOW] = bytes(random.getrandbits(8) for _ in range(WINDOW))
        if e.fix:
            e.fix(regs, mem)
        flags = random_flags(True)
        vectors.append((wrap(code, flags), regs, bytes(mem)))
        meta.append((e, flags))
    out = difftest_mem.run_oracle(vectors, pro, epi)
    rows = []
    for (code, regs, mem), (e, flags), (o, omem) in zip(vectors, meta, out):
        if o is None:
            continue
        mask, cmp_regs = e.rule(regs)
        if not cmp_regs:
            continue
        if e.text.startswith("cmpxchg"):
            mask = 0
        rows.append(("mem_" + e.fam, e.text, code, regs, flags, o, mask, mem[:WINDOW], omem[:WINDOW]))
    return cap(rows, "memory")


def rust_row(fam, text, code, regs, flags, o, mask, mem_in, mem_out):
    regs_in = ", ".join(f"({i}, {v:#x})" for i, v in enumerate(regs[:9]) if v)
    regs_out = ", ".join(f"({i}, {o[i]:#x})" for i in range(9) if o[i] != regs[i])
    asm = text.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "; ").replace("\t", "")
    shellcode = (code + b"\xf4").hex()
    flags_out = o[9] & mask
    return (f'    v("{fam}", "{asm}", "{shellcode}", &[{regs_in}], {flags:#x}, '
            f'&[{regs_out}], {flags_out:#x}, {mask:#x}, "{mem_in.hex()}", "{mem_out.hex()}"),')


def main():
    random.seed(2026)
    build_oracles()
    pro, epi = build_wrappers()
    rows = register_vectors(pro, epi) + memory_vectors(pro, epi)
    lines = [
        "//! CPU-recorded x86-64 vectors. Generated by `scripts/rosetta-difftest/gen_vectors.py`",
        "//! from the real CPU (through Rosetta); do not edit by hand.",
        "//!",
        "//! Each vector: family, assembly, the complete shellcode (`push imm32; popfq;",
        "//! <insn>; pushfq; pop r11; hlt`), the non-zero input registers (index into",
        "//! rax rbx rcx rdx rsi rdi r8 r9 r10), the input flags, the registers the CPU",
        "//! changed, the output flags already masked to the architecturally defined",
        "//! ones, that mask, and the 128-byte data window before and after (memory",
        "//! vectors only). See `cpu_vector_tests.rs` for the runner.",
        "",
        "use super::cpu_vector_tests::{v, Vector};",
        "",
        "#[rustfmt::skip]",
        "pub(super) const VECTORS: &[Vector] = &[",
    ] + [rust_row(*r) for r in rows] + ["];", ""]
    with open(OUT, "w") as f:
        f.write("\n".join(lines))
    fams = {}
    for r in rows:
        fams[r[0]] = fams.get(r[0], 0) + 1
    print(f"{len(rows)} vectors -> {os.path.relpath(OUT)}")
    print(" ".join(f"{k}={n}" for k, n in sorted(fams.items())))


if __name__ == "__main__":
    main()
