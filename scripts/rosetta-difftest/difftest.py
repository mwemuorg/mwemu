"""Differential test: x86-64 instructions on the real CPU (Rosetta) vs pymwemu.

Each vector = one instruction (or tiny snippet) + 10 input registers + input flags.
Both sides execute the exact same bytes:  push imm32 ; popfq ; <insn> ; pushfq ; pop r11
"""
import argparse, collections, os, random, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from asmlib import asm  # noqa: E402
NAMES = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"]
CF, PF, AF, ZF, SF, DF, OF = 1 << 0, 1 << 2, 1 << 4, 1 << 6, 1 << 7, 1 << 10, 1 << 11
ARITH = CF | PF | AF | ZF | SF | OF
ALL = ARITH | DF
FLAG_NAMES = [(CF, "CF"), (PF, "PF"), (AF, "AF"), (ZF, "ZF"), (SF, "SF"), (DF, "DF"), (OF, "OF")]
SENTINEL = 0x1122334455667788

R64 = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10"]
R32 = ["eax", "ebx", "ecx", "edx", "esi", "edi", "r8d", "r9d", "r10d"]
R16 = ["ax", "bx", "cx", "dx", "si", "di", "r8w", "r9w", "r10w"]
R8L = ["al", "bl", "cl", "dl", "ah", "bh", "ch", "dh"]
R8R = ["al", "bl", "cl", "dl", "sil", "dil", "r8b", "r9b", "r10b"]
SPECIAL = [0, 1, 2, 3, 0x7f, 0x80, 0x81, 0xff, 0x100, 0x7fff, 0x8000, 0xffff, 0x10000, 0x7fffffff,
           0x80000000, 0xffffffff, 0x100000000, 0x7fffffffffffffff, 0x8000000000000000, 0xffffffffffffffff]
COUNTS = [0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 255]
CC = ["o", "no", "b", "ae", "e", "ne", "be", "a", "s", "ns", "p", "np", "l", "ge", "le", "g"]


def rnd64():
    k = random.random()
    if k < 0.35:
        return random.choice(SPECIAL)
    if k < 0.5:
        return random.getrandbits(8)
    if k < 0.6:
        return random.getrandbits(16)
    if k < 0.75:
        return random.getrandbits(32)
    if k < 0.85:
        return (random.getrandbits(64) & ~0xffffffff) | (random.choice(SPECIAL) & 0xffffffff)
    return random.getrandbits(64)


def regs_of(size):
    return {64: R64, 32: R32, 16: R16}[size] if size != 8 else random.choice([R8L, R8R])


def reg_index(name):
    """Index into NAMES of the full register that `name` aliases."""
    n = name
    if n in R64:
        return R64.index(n)
    for lst in (R32, R16, R8R):
        if n in lst:
            return lst.index(n)
    return {"ah": 0, "bh": 1, "ch": 2, "dh": 3}[n]


def width_mask(size):
    return (1 << size) - 1


def read_sub(regs, name, size):
    v = regs[reg_index(name)]
    if name in ("ah", "bh", "ch", "dh"):
        return (v >> 8) & 0xff
    return v & width_mask(size)


# ---- flag-mask rules -------------------------------------------------------
def m_const(m):
    return lambda regs: (m, True)


def shift_rule(size, count):
    c = count & (0x3f if size == 64 else 0x1f)
    if c == 0:
        return ALL, True
    m = ARITH & ~AF
    if c != 1:
        m &= ~OF
    if c >= size:
        m &= ~CF
    return m | DF, True


def rot_rule(size, count):
    c = count & (0x3f if size == 64 else 0x1f)
    if c == 0:
        return ALL, True
    return (ALL if c == 1 else ALL & ~OF), True


def dshift_rule(size, count):
    c = count & (0x3f if size == 64 else 0x1f)
    if c == 0:
        return ALL, True
    if c > size:
        return 0, False
    m = ARITH & ~AF
    if c != 1:
        m &= ~OF
    return m | DF, True


class Entry:
    def __init__(self, fam, text, rule=m_const(ALL), fix=None):
        self.fam, self.text, self.rule, self.fix = fam, text, rule, fix


def build_catalog(per):
    E = []

    def add(fam, text, rule=m_const(ALL), fix=None):
        E.append(Entry(fam, text, rule, fix))

    def pick(lst, k=2):
        return [random.choice(lst) for _ in range(k)]

    # ALU reg,reg / reg,imm
    for mn in ["add", "adc", "sub", "sbb", "cmp", "and", "or", "xor", "test"]:
        rule = m_const(ALL & ~AF) if mn in ("and", "or", "xor", "test") else m_const(ALL)
        for size in (64, 32, 16, 8):
            for _ in range(per):
                R = regs_of(size)
                a, b = pick(R)
                add("alu", f"{mn} {a}, {b}", rule)
            R = regs_of(size)
            a = random.choice(R)
            imm = random.choice([0, 1, 0x7f, 0x80, 0xff, -1, -128, 0x7fff, 0x12345678, -0x12345678, 0x7fffffff])
            if size == 8:
                imm &= 0xff
            elif size == 16:
                imm &= 0xffff
            elif size == 32:
                imm &= 0xffffffff
            else:
                imm = (imm & 0xffffffff) if -0x80000000 <= imm <= 0x7fffffff else imm
                if imm > 0x7fffffff:
                    imm -= 0x100000000
            add("alu", f"{mn} {a}, {imm}", rule)
    for mn in ["inc", "dec", "neg", "not"]:
        for size in (64, 32, 16, 8):
            for a in pick(regs_of(size)):
                add("alu", f"{mn} {a}")
    # shifts and rotates
    for mn in ["shl", "shr", "sar", "rol", "ror", "rcl", "rcr"]:
        rule_f = shift_rule if mn in ("shl", "shr", "sar") else rot_rule
        for size in (64, 32, 16, 8):
            for cnt in random.sample(COUNTS, min(per + 3, len(COUNTS))) + [1]:
                a = random.choice(regs_of(size))
                add("shift", f"{mn} {a}, {cnt}", (lambda s, c, rf: lambda regs: rf(s, c))(size, cnt, rule_f))
            R = [r for r in regs_of(size) if r not in ("rcx", "ecx", "cx", "cl", "ch")]
            for a in pick(R):
                add("shift", f"{mn} {a}, cl", (lambda s, rf: lambda regs: rf(s, regs[2] & 0xff))(size, rule_f),
                    fix=lambda regs: regs.__setitem__(2, (regs[2] & ~0xff) | random.choice(COUNTS + [random.getrandbits(8)])))
    for mn in ["shld", "shrd"]:
        for size in (64, 32, 16):
            R = regs_of(size)
            for cnt in random.sample(COUNTS, min(per + 3, len(COUNTS))) + [1]:
                a, b = pick(R)
                add("dshift", f"{mn} {a}, {b}, {cnt}", (lambda s, c: lambda regs: dshift_rule(s, c))(size, cnt))
            R2 = [r for r in R if r not in ("rcx", "ecx", "cx")]
            for _ in range(per):
                a, b = pick(R2)
                add("dshift", f"{mn} {a}, {b}, cl", (lambda s: lambda regs: dshift_rule(s, regs[2] & 0xff))(size),
                    fix=lambda regs: regs.__setitem__(2, (regs[2] & ~0xff) | random.choice(COUNTS + [random.getrandbits(8)])))
    # mul / imul / div / idiv
    for size in (64, 32, 16, 8):
        R = regs_of(size)
        for mn in ["mul", "imul"]:
            for a in pick(R):
                add("mul", f"{mn} {a}", m_const(CF | OF | DF))
        if size != 8:
            for _ in range(per):
                a, b = pick(R)
                add("mul", f"imul {a}, {b}", m_const(CF | OF | DF))
                imm = random.choice([1, -1, 2, 0x7f, -128, 0x7fff, -0x8000, 0x12345, 0x7fffffff, -0x7fffffff])
                if size == 16:
                    imm = max(-0x8000, min(0x7fff, imm))
                add("mul", f"imul {a}, {b}, {imm}", m_const(CF | OF | DF))

        def div_fix(size, src):
            def f(regs):
                idx = reg_index(src)
                if random.random() < 0.85 and read_sub(regs, src, size) == 0:
                    regs[idx] |= 1
                if random.random() < 0.8:
                    d = read_sub(regs, src, size) or 1
                    if size == 8:
                        regs[0] = (regs[0] & ~0xffff) | (random.getrandbits(16) % (d << 8 if random.random() < 0.5 else 256))
                    else:
                        hi = random.randrange(d) if random.random() < 0.5 else random.getrandbits(size) % max(d >> 1, 1)
                        regs[3] = (regs[3] & ~width_mask(size)) | hi
            return f

        for mn in ["div", "idiv"]:
            for a in pick(R, 3):
                add("div", f"{mn} {a}", m_const(DF), fix=div_fix(size, a))
    # bit ops
    for mn in ["bt", "bts", "btr", "btc"]:
        for size in (64, 32, 16):
            R = regs_of(size)
            for _ in range(per):
                a, b = pick(R)
                add("bit", f"{mn} {a}, {b}", m_const(CF | ZF | DF))
            a = random.choice(R)
            add("bit", f"{mn} {a}, {random.choice([0, 1, 7, 15, 16, 31, 32, 63, 255])}", m_const(CF | ZF | DF))
    for mn in ["bsf", "bsr"]:
        for size in (64, 32, 16):
            R = regs_of(size)
            for _ in range(per):
                a, b = pick(R)
                add("bit", f"{mn} {a}, {b}", (lambda s, src: lambda regs: (ZF | DF, read_sub(regs, src, s) != 0))(size, b))
    for mn, m in [("popcnt", ALL), ("lzcnt", CF | ZF | DF), ("tzcnt", CF | ZF | DF)]:
        for size in (64, 32, 16):
            R = regs_of(size)
            for _ in range(per):
                a, b = pick(R)
                add("bit", f"{mn} {a}, {b}", m_const(m))
    # condition codes
    for cc in CC:
        for size in (64, 32, 16):
            a, b = pick(regs_of(size))
            add("cc", f"cmov{cc} {a}, {b}")
        add("cc", f"set{cc} {random.choice(regs_of(8))}")
        add("cc", f"j{cc} 1f\n\tinc r10\n1:")
    for mn in ["jrcxz", "jecxz", "loop", "loope", "loopne"]:
        add("cc", f"{mn} 1f\n\tinc r10\n1:")
    # moves / extensions / swaps
    for size in (64, 32, 16, 8):
        R = regs_of(size)
        for _ in range(per):
            a, b = pick(R)
            add("mov", f"mov {a}, {b}")
            add("mov", f"xchg {a}, {b}")
        a = random.choice(R)
        imm = random.choice([0, -1, 0x7f, 0x80, 0xff, 0x12345678, -0x12345678])
        if size == 8:
            imm &= 0xff
        elif size == 16:
            imm &= 0xffff
        elif size == 32:
            imm &= 0xffffffff
        add("mov", f"mov {a}, {imm}")
        if size >= 16:
            for _ in range(per):
                a, b = pick(R)
                add("alu", f"xadd {a}, {b}")
                add("alu", f"cmpxchg {a}, {b}")
    add("mov", "movabs rax, 0x123456789abcdef0")
    add("mov", "mov rbx, -1")
    add("mov", "mov r8, 0xffffffff")
    add("mov", "xchg eax, eax")
    add("mov", "xchg r8d, r8d")
    add("mov", "xchg rax, r8")
    add("mov", "xchg eax, r9d")
    for a in pick(R64, per):
        add("mov", f"movsxd {a}, {random.choice(R32)}")
    for dst_size in (64, 32, 16):
        for src_size in (16, 8):
            if src_size >= dst_size:
                continue
            for mn in ["movzx", "movsx"]:
                for _ in range(per):
                    dst = random.choice(regs_of(dst_size))
                    if src_size == 8:
                        legacy = dst in ("rax", "rbx", "rcx", "rdx", "eax", "ebx", "ecx", "edx", "ax", "bx", "cx", "dx")
                        src = random.choice(random.choice([R8L, R8R]) if legacy else R8R)
                    else:
                        src = random.choice(R16)
                    add("mov", f"{mn} {dst}, {src}")
    for mn in ["cbw", "cwde", "cdqe", "cwd", "cdq", "cqo", "lahf", "sahf", "cmc", "clc", "stc", "cld", "std"]:
        add("misc", mn)
    for a in pick(R64, per) + pick(R32, per):
        add("mov", f"bswap {a}")
    add("mov", "lea rax, [rbx + rcx*4 + 0x10]")
    add("mov", "lea eax, [ebx + ecx*2 - 5]")
    add("mov", "lea ax, [rbx + rcx]")
    add("mov", "lea r8, [rsi + rdi*8 - 0x1000]")
    add("mov", "lea r9d, [r8d + r10d*1 + 0x7fffffff]")
    add("mov", "lea rdx, [rax*8 + 3]")
    # BMI / ADX
    for size in (64, 32):
        R = regs_of(size)
        for _ in range(per):
            a, b, c = pick(R, 3)
            add("bmi", f"andn {a}, {b}, {c}", m_const(ALL & ~AF & ~PF))
            add("bmi", f"bextr {a}, {b}, {c}", m_const(CF | ZF | OF | DF))
            add("bmi", f"blsi {a}, {b}", m_const(CF | ZF | SF | OF | DF))
            add("bmi", f"blsmsk {a}, {b}", m_const(CF | ZF | SF | OF | DF))
            add("bmi", f"blsr {a}, {b}", m_const(CF | ZF | SF | OF | DF))
            add("bmi", f"bzhi {a}, {b}, {c}", m_const(CF | ZF | SF | OF | DF))
            add("bmi", f"pdep {a}, {b}, {c}")
            add("bmi", f"pext {a}, {b}, {c}")
            add("bmi", f"sarx {a}, {b}, {c}")
            add("bmi", f"shlx {a}, {b}, {c}")
            add("bmi", f"shrx {a}, {b}, {c}")
            add("bmi", f"rorx {a}, {b}, {random.choice(COUNTS)}")
            add("bmi", f"adcx {a}, {b}")
            add("bmi", f"adox {a}, {b}")
            d = [r for r in R if r not in ("rdx", "edx")]
            a, b, c = pick(d, 3)
            add("bmi", f"mulx {a}, {b}, {c}")
    return E


# ---- runners ---------------------------------------------------------------
def build_wrappers():
    pro = asm(["push rbx", "push rbp", "push r12", "push r13", "push r14", "push r15", "mov r15, rdi"] +
              [f"mov {n}, [r15 + {8 * i}]" for i, n in enumerate(NAMES)])
    epi = asm([f"mov [r15 + {8 * i}], {n}" for i, n in enumerate(NAMES)] +
              ["pop r15", "pop r14", "pop r13", "pop r12", "pop rbp", "pop rbx", "ret"])
    return b"".join(pro), b"".join(epi)


def wrap(insn_bytes, flags):
    return bytes([0x68]) + flags.to_bytes(4, "little") + b"\x9d" + insn_bytes + b"\x9c\x41\x5b"


def run_oracle(vectors, pro, epi):
    lines = []
    for code, regs in vectors:
        lines.append((pro + code + epi).hex() + " " + " ".join(f"{r:x}" for r in regs))
    p = subprocess.run([os.path.join(HERE, "oracle")], input="\n".join(lines) + "\n", capture_output=True, text=True)
    out = []
    for ln in p.stdout.splitlines():
        parts = ln.split()
        out.append([int(x, 16) for x in parts[1:]] if parts[0] == "OK" else None)
    assert len(out) == len(vectors), (len(out), len(vectors), p.stderr[:200])
    return out


def run_mwemu(code, regs, pymwemu):
    emu = pymwemu.init64()
    try:
        emu.load_code_bytes(code + b"\xf4")
        for n, v in zip(NAMES, regs):
            emu.set_reg(n, v)
        emu.run(None)
        out = [emu.get_reg(n) for n in NAMES]
    except Exception:
        return None
    if out[9] == SENTINEL:
        return None
    return out


def fl(v):
    return " ".join(n for b, n in FLAG_NAMES if v & b) or "-"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--per", type=int, default=2, help="random register combos per family/size")
    ap.add_argument("--n", type=int, default=5, help="random states per instruction")
    ap.add_argument("--fam", default=None, help="comma-separated families to run")
    ap.add_argument("--show", type=int, default=2, help="mismatches to print per instruction")
    args = ap.parse_args()
    random.seed(args.seed)
    sys.path.insert(0, os.environ.get("PYMWEMU_PATH", ""))
    import pymwemu

    pro, epi = build_wrappers()
    cat = build_catalog(args.per)
    if args.fam:
        fams = set(args.fam.split(","))
        cat = [e for e in cat if e.fam in fams]
    texts = [e.text for e in cat]
    codes = asm(texts)
    vectors, meta = [], []
    for e, code in zip(cat, codes):
        for _ in range(args.n):
            regs = [rnd64() for _ in range(9)] + [SENTINEL]
            if e.fix:
                e.fix(regs)
            flags = 0
            for b in (CF, PF, AF, ZF, SF, OF):
                if random.random() < 0.5:
                    flags |= b
            vectors.append((wrap(code, flags), regs))
            meta.append((e, flags))
    print(f"{len(cat)} instructions, {len(vectors)} vectors")
    oracle = run_oracle(vectors, pro, epi)
    ill = collections.Counter()
    results = collections.OrderedDict()
    for (code, regs), (e, flags), o in zip(vectors, meta, oracle):
        mask, cmp_regs = e.rule(regs)
        m = run_mwemu(code, regs, pymwemu)
        r = results.setdefault(e.text, {"fam": e.fam, "n": 0, "bad": [], "skip": 0})
        r["n"] += 1
        if o is None and m is None:
            continue
        if o is None:
            ill[e.text] += 1
            r["bad"].append(("oracle EXC, mwemu completed", regs, flags, o, m, mask))
            continue
        if m is None:
            r["bad"].append(("mwemu did not complete (exception?)", regs, flags, o, m, mask))
            continue
        diffs = []
        if cmp_regs:
            for i in range(9):
                if o[i] != m[i]:
                    diffs.append(f"{NAMES[i]}: cpu={o[i]:#x} mwemu={m[i]:#x}")
        if (o[9] ^ m[9]) & mask:
            diffs.append(f"flags: cpu=[{fl(o[9] & mask)}] mwemu=[{fl(m[9] & mask)}]")
        if diffs:
            r["bad"].append(("; ".join(diffs), regs, flags, o, m, mask))
    bad_ins = [(t, r) for t, r in results.items() if r["bad"]]
    print(f"\n==== {len(bad_ins)} instructions with mismatches, {sum(len(r['bad']) for _, r in bad_ins)} vectors ====")
    byfam = collections.Counter(r["fam"] for _, r in bad_ins)
    print("by family:", dict(byfam))
    for t, r in bad_ins:
        print(f"\n--- {t!r} [{r['fam']}] {len(r['bad'])}/{r['n']} mismatches")
        for what, regs, flags, o, m, mask in r["bad"][: args.show]:
            ins = " ".join(f"{n}={v:#x}" for n, v in zip(NAMES[:9], regs) if v != 0)
            print(f"    in: flags=[{fl(flags)}] {ins}")
            print(f"    {what}")


if __name__ == "__main__":
    main()
