"""Memory-operand and string-instruction differential test: real CPU (Rosetta) vs pymwemu."""
import argparse, collections, os, random, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from asmlib import asm  # noqa: E402
from difftest import (NAMES, CF, PF, AF, ZF, SF, DF, OF, ARITH, ALL, SENTINEL, COUNTS, CC, rnd64, shift_rule,  # noqa: E402
                      rot_rule, dshift_rule, m_const, build_wrappers, wrap, fl, reg_index, width_mask)

BASE = 0x300000000
MEMSZ = 1024
MP = {64: "qword ptr", 32: "dword ptr", 16: "word ptr", 8: "byte ptr"}
ADDRS = ["[rsi]", "[rdi + 8]", "[rbx + rcx*2 + 1]", "[rsi - 16]", "[rdi + rcx*8]", "[rbx + 0x30]", "[rsi + rcx]"]
D = {64: ["rax", "rdx", "r8", "r9", "r10"], 32: ["eax", "edx", "r8d", "r9d", "r10d"],
     16: ["ax", "dx", "r8w", "r9w", "r10w"], 8: ["al", "dl", "ah", "dh", "r8b", "r9b", "r10b"]}


class Entry:
    def __init__(self, fam, text, rule=m_const(ALL), fix=None):
        self.fam, self.text, self.rule, self.fix = fam, text, rule, fix


WINDOW = MEMSZ  # bytes of the buffer the pinned pointers stay inside


def pin(regs):
    regs[4] = BASE + WINDOW // 2 + 16 * random.randrange(4)
    regs[5] = BASE + WINDOW // 4 + 16 * random.randrange(4)
    regs[1] = BASE + WINDOW // 8 + random.randrange(16)
    regs[2] = random.choice(COUNTS) if random.random() < 0.3 else random.randrange(WINDOW // 25)


def off_of(regs, addr):
    """Byte offset inside the buffer addressed by `addr` for the pinned registers."""
    rsi, rdi, rbx, rcx = regs[4] - BASE, regs[5] - BASE, regs[1] - BASE, regs[2]
    return {"[rsi]": rsi, "[rdi + 8]": rdi + 8, "[rbx + rcx*2 + 1]": rbx + rcx * 2 + 1, "[rsi - 16]": rsi - 16,
            "[rdi + rcx*8]": rdi + rcx * 8, "[rbx + 0x30]": rbx + 0x30, "[rsi + rcx]": rsi + rcx}[addr]


def imm_for(size):
    imm = random.choice([0, 1, 0x7f, 0x80, 0xff, -1, -128, 0x7fff, 0x12345678, -0x12345678, 0x7fffffff])
    if size == 8:
        return imm & 0xff
    if size == 16:
        return imm & 0xffff
    if size == 32:
        return imm & 0xffffffff
    return imm if -0x80000000 <= imm <= 0x7fffffff else imm - 0x100000000


def build_catalog(per):
    E = []

    def add(fam, text, rule=m_const(ALL), fix=None):
        E.append(Entry(fam, text, rule, fix))

    logic = ("and", "or", "xor", "test")
    for mn in ["add", "adc", "sub", "sbb", "cmp", "and", "or", "xor", "test"]:
        rule = m_const(ALL & ~AF) if mn in logic else m_const(ALL)
        for size in (64, 32, 16, 8):
            for _ in range(per):
                d, a = random.choice(D[size]), random.choice(ADDRS)
                add("alu", f"{mn} {MP[size]} {a}, {d}", rule)
                if mn != "test":
                    add("alu", f"{mn} {d}, {MP[size]} {a}", rule)
                add("alu", f"{mn} {MP[size]} {a}, {imm_for(size)}", rule)
    for mn in ["inc", "dec", "neg", "not"]:
        for size in (64, 32, 16, 8):
            add("alu", f"{mn} {MP[size]} {random.choice(ADDRS)}")
    for mn in ["shl", "shr", "sar", "rol", "ror", "rcl", "rcr"]:
        rf = shift_rule if mn in ("shl", "shr", "sar") else rot_rule
        for size in (64, 32, 16, 8):
            for cnt in random.sample(COUNTS, per) + [1]:
                add("shift", f"{mn} {MP[size]} {random.choice(ADDRS)}, {cnt}", (lambda s, c, f: lambda r: f(s, c))(size, cnt, rf))
            add("shift", f"{mn} {MP[size]} {random.choice(ADDRS)}, cl", (lambda s, f: lambda r: f(s, r[2] & 0xff))(size, rf))
    for mn in ["shld", "shrd"]:
        for size in (64, 32, 16):
            d = random.choice(D[size])
            cnt = random.choice(COUNTS)
            add("dshift", f"{mn} {MP[size]} {random.choice(ADDRS)}, {d}, {cnt}", (lambda s, c: lambda r: dshift_rule(s, c))(size, cnt))
            add("dshift", f"{mn} {MP[size]} {random.choice(ADDRS)}, {d}, cl", (lambda s: lambda r: dshift_rule(s, r[2] & 0xff))(size))

    def div_fix(size):
        def f(regs, mem):
            off = regs[4] - BASE
            if random.random() < 0.85 and not any(mem[off:off + size // 8]):
                mem[off] |= 1
            if random.random() < 0.8:
                d = int.from_bytes(mem[off:off + size // 8], "little") or 1
                if size == 8:
                    regs[0] = (regs[0] & ~0xffff) | (random.getrandbits(16) % max(d << 8, 1))
                else:
                    regs[3] = (regs[3] & ~width_mask(size)) | (random.randrange(d) if random.random() < 0.5 else random.getrandbits(size) % max(d >> 1, 1))
        return f

    for size in (64, 32, 16, 8):
        for mn in ["mul", "imul"]:
            add("mul", f"{mn} {MP[size]} [rsi]", m_const(CF | OF | DF))
        for mn in ["div", "idiv"]:
            add("div", f"{mn} {MP[size]} [rsi]", m_const(DF), fix=div_fix(size))
        if size != 8:
            d = random.choice(D[size])
            add("mul", f"imul {d}, {MP[size]} {random.choice(ADDRS)}", m_const(CF | OF | DF))
            add("mul", f"imul {d}, {MP[size]} {random.choice(ADDRS)}, {imm_for(16 if size == 16 else 32) if size == 16 else random.choice([-1, 2, 0x7fff, -0x8000, 0x12345, 0x7fffffff])}", m_const(CF | OF | DF))

    def bit_fix(d, size):
        def f(regs, mem):
            v = random.randrange(-2 * WINDOW, 2 * WINDOW) & width_mask(size)
            i = reg_index(d)
            regs[i] = (regs[i] & ~width_mask(size)) | v
        return f

    for mn in ["bt", "bts", "btr", "btc"]:
        for size in (64, 32, 16):
            d = random.choice(D[size])
            add("bit", f"{mn} {MP[size]} {random.choice(ADDRS)}, {d}", m_const(CF | ZF | DF), fix=bit_fix(d, size))
            add("bit", f"{mn} {MP[size]} {random.choice(ADDRS)}, {random.choice([0, 1, 7, 15, 31, 63, 255])}", m_const(CF | ZF | DF))

    def nonzero_src(regs, mem):
        off = regs[4] - BASE
        if random.random() < 0.9:
            mem[off + random.randrange(2)] |= random.randrange(1, 256)

    for mn, m in [("bsf", ZF | DF), ("bsr", ZF | DF), ("popcnt", ALL), ("lzcnt", CF | ZF | DF), ("tzcnt", CF | ZF | DF)]:
        for size in (64, 32, 16):
            add("bit", f"{mn} {random.choice(D[size])}, {MP[size]} [rsi]", m_const(m), fix=nonzero_src)
    for size in (64, 32, 16, 8):
        for _ in range(per):
            d, a = random.choice(D[size]), random.choice(ADDRS)
            add("xchg", f"xadd {MP[size]} {a}, {d}")
            add("xchg", f"cmpxchg {MP[size]} {a}, {d}")
            add("xchg", f"xchg {MP[size]} {a}, {d}")
            add("mov", f"mov {d}, {MP[size]} {a}")
            add("mov", f"mov {MP[size]} {a}, {d}")
            add("mov", f"mov {MP[size]} {a}, {imm_for(size)}")
    add("xchg", "cmpxchg8b [rsi]")
    add("xchg", "cmpxchg16b [rsi]")
    for dst_size in (64, 32, 16):
        for src_size in (16, 8):
            if src_size < dst_size:
                for mn in ["movzx", "movsx"]:
                    add("mov", f"{mn} {random.choice(D[dst_size])}, {MP[src_size]} {random.choice(ADDRS)}")
    add("mov", f"movsxd {random.choice(D[64])}, dword ptr {random.choice(ADDRS)}")
    for size in (64, 32, 16):
        d = random.choice(D[size])
        add("mov", f"movbe {d}, {MP[size]} {random.choice(ADDRS)}")
        add("mov", f"movbe {MP[size]} {random.choice(ADDRS)}, {d}")
    add("mov", "push qword ptr [rsi]\n\tpop qword ptr [rdi + 8]")
    add("mov", "push qword ptr [rsi]\n\tpop rax")
    add("mov", "push rdx\n\tpop qword ptr [rbx + 0x30]")
    for cc in random.sample(CC, 6):
        add("cc", f"set{cc} byte ptr {random.choice(ADDRS)}")
        add("cc", f"cmov{cc} {random.choice(D[random.choice((64, 32, 16))])}, {MP[64]} [rsi]".replace("qword ptr", "") if False else f"cmov{cc} {random.choice(D[64])}, qword ptr {random.choice(ADDRS)}")
    add("cc", f"cmovne {random.choice(D[32])}, dword ptr {random.choice(ADDRS)}")
    add("cc", f"cmovs {random.choice(D[16])}, word ptr {random.choice(ADDRS)}")

    def str_fix(regs, mem):
        top = WINDOW // 40
        regs[2] = random.choice([0, 1, 2, 3, 5, 8, top, random.randrange(top + 1)])
        if random.random() < 0.5:
            q = WINDOW // 4
            mem[q:q + q // 4] = mem[2 * q:2 * q + q // 4]
        for _ in range(4):
            mem[random.randrange(WINDOW)] = 0

    for sz in "bwdq":
        for mn in ["movs", "stos", "lods", "scas", "cmps"]:
            add("str", f"{mn}{sz}", fix=str_fix)
        for pre, mn in [("rep", "movs"), ("rep", "stos"), ("rep", "lods"), ("repe", "cmps"), ("repne", "cmps"), ("repe", "scas"), ("repne", "scas")]:
            add("str", f"{pre} {mn}{sz}", fix=str_fix)
    add("misc", "xlatb")
    add("misc", "crc32 eax, byte ptr [rsi]")
    add("misc", "crc32 eax, word ptr [rsi]")
    add("misc", "crc32 eax, dword ptr [rsi]")
    add("misc", "crc32 rax, qword ptr [rsi]")
    add("misc", "crc32 eax, edx")
    add("misc", "crc32 rax, r8")
    add("misc", "nop dword ptr [rsi]")
    return E


def run_oracle(vectors, pro, epi):
    lines = [(pro + code + epi).hex() + " " + " ".join(f"{r:x}" for r in regs) + " " + mem.hex() for code, regs, mem in vectors]
    p = subprocess.run([os.path.join(HERE, "oracle2")], input="\n".join(lines) + "\n", capture_output=True, text=True)
    out = []
    for ln in p.stdout.splitlines():
        parts = ln.split()
        if parts[0] == "OK":
            out.append(([int(x, 16) for x in parts[1:11]], bytes.fromhex(parts[11])))
        else:
            out.append((None, None))
    assert len(out) == len(vectors), (len(out), len(vectors), p.stderr[:300])
    return out


def run_mwemu(code, regs, mem, pymwemu):
    emu = pymwemu.init64()
    try:
        emu.load_code_bytes(code + b"\xf4")
        emu.alloc_at("diffbuf", BASE, 0x10000)
        emu.write_bytes(BASE, list(mem))
        for n, v in zip(NAMES, regs):
            emu.set_reg(n, v)
        emu.run(None)
        out = [emu.get_reg(n) for n in NAMES]
        mo = bytes(emu.read_bytes(BASE, MEMSZ))
    except Exception:
        return None, None
    if out[9] == SENTINEL:
        return None, None
    return out, mo


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--per", type=int, default=2)
    ap.add_argument("--n", type=int, default=5)
    ap.add_argument("--fam", default=None)
    ap.add_argument("--show", type=int, default=2)
    args = ap.parse_args()
    random.seed(args.seed)
    import pymwemu

    pro, epi = build_wrappers()
    cat = build_catalog(args.per)
    if args.fam:
        cat = [e for e in cat if e.fam in set(args.fam.split(","))]
    codes = asm([e.text for e in cat])
    vectors, meta = [], []
    for e, code in zip(cat, codes):
        for _ in range(args.n):
            regs = [rnd64() for _ in range(9)] + [SENTINEL]
            pin(regs)
            mem = bytearray(random.getrandbits(8) for _ in range(MEMSZ))
            if random.random() < 0.3:
                mem[512:512 + 16] = bytes(16) if random.random() < 0.5 else b"\xff" * 16
            if e.fix:
                e.fix(regs, mem)
            flags = 0
            for b in (CF, PF, AF, ZF, SF, OF, DF):
                if random.random() < 0.5:
                    flags |= b
            vectors.append((wrap(code, flags), regs, bytes(mem)))
            meta.append((e, flags))
    print(f"{len(cat)} instructions, {len(vectors)} vectors")
    oracle = run_oracle(vectors, pro, epi)
    results = collections.OrderedDict()
    for (code, regs, mem), (e, flags), (o, omem) in zip(vectors, meta, oracle):
        mask, cmp_regs = e.rule(regs)
        m, mmem = run_mwemu(code, regs, mem, pymwemu)
        r = results.setdefault(e.text, {"fam": e.fam, "n": 0, "bad": []})
        r["n"] += 1
        if o is None and m is None:
            continue
        if o is None:
            r["bad"].append(("oracle EXC, mwemu completed", regs, flags))
            continue
        if m is None:
            r["bad"].append(("mwemu did not complete (exception?)", regs, flags))
            continue
        diffs = []
        if cmp_regs:
            for i in range(9):
                if o[i] != m[i]:
                    diffs.append(f"{NAMES[i]}: cpu={o[i]:#x} mwemu={m[i]:#x}")
        if (o[9] ^ m[9]) & mask:
            diffs.append(f"flags: cpu=[{fl(o[9] & mask)}] mwemu=[{fl(m[9] & mask)}]")
        if omem != mmem:
            bad = [i for i in range(MEMSZ) if omem[i] != mmem[i]]
            lo, hi = bad[0], bad[-1] + 1
            diffs.append(f"mem[{lo:#x}:{hi:#x}] ({len(bad)} bytes) cpu={omem[lo:min(hi, lo + 16)].hex()} mwemu={mmem[lo:min(hi, lo + 16)].hex()}")
        if diffs:
            r["bad"].append(("; ".join(diffs), regs, flags))
    bad_ins = [(t, r) for t, r in results.items() if r["bad"]]
    print(f"\n==== {len(bad_ins)} instructions with mismatches, {sum(len(r['bad']) for _, r in bad_ins)} vectors ====")
    print("by family:", dict(collections.Counter(r["fam"] for _, r in bad_ins)))
    for t, r in bad_ins:
        print(f"\n--- {t!r} [{r['fam']}] {len(r['bad'])}/{r['n']} mismatches")
        for what, regs, flags in r["bad"][: args.show]:
            ins = " ".join(f"{n}={v:#x}" for n, v in zip(NAMES[:9], regs))
            print(f"    in: flags=[{fl(flags)}] {ins}")
            print(f"    {what}")


if __name__ == "__main__":
    main()
