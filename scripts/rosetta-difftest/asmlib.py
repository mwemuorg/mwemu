"""Assemble Intel-syntax x86-64 instructions with clang and return their bytes."""
import os, re, subprocess, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
LINE = re.compile(r"^\s*[0-9a-f]+:\s+((?:[0-9a-f]{2}\s)+)")
SYM = re.compile(r"^[0-9a-f]+\s+<(\w+)>:")

def asm(instrs):
    """instrs: list of Intel-syntax strings -> list of bytes objects (one per instruction)."""
    src = ".intel_syntax noprefix\n.text\n"
    for i, ins in enumerate(instrs):
        src += f".globl lbl{i}\nlbl{i}:\n\t{ins}\n"
    src += f".globl lbl{len(instrs)}\nlbl{len(instrs)}:\n\tnop\n"
    with tempfile.TemporaryDirectory(dir=HERE) as d:
        s = os.path.join(d, "a.s"); o = os.path.join(d, "a.o")
        with open(s, "w") as f:
            f.write(src)
        subprocess.run(["clang", "-arch", "x86_64", "-c", s, "-o", o], check=True)
        out = subprocess.run(["objdump", "-d", o], check=True, capture_output=True, text=True).stdout
    chunks = {}
    cur = None
    for ln in out.splitlines():
        m = SYM.match(ln)
        if m:
            cur = m.group(1)
            chunks.setdefault(cur, bytearray())
            continue
        m = LINE.match(ln)
        if m and cur is not None:
            chunks[cur] += bytes.fromhex(m.group(1).replace(" ", ""))
    res = []
    for i in range(len(instrs)):
        b = bytes(chunks.get(f"lbl{i}", b""))
        if not b:
            raise RuntimeError(f"no bytes for {instrs[i]!r}")
        res.append(b)
    return res

if __name__ == "__main__":
    for ins, b in zip(["add rax, rbx", "shl eax, 1", "imul rax, rdx", "cmovl r8d, r9d", "mov rax, [r15]", "pop r11"],
                      asm(["add rax, rbx", "shl eax, 1", "imul rax, rdx", "cmovl r8d, r9d", "mov rax, [r15]", "pop r11"])):
        print(f"{ins:20s} {b.hex()}")
