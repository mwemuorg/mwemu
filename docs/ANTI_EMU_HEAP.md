# Anti-Emulation: Windows Heap Structure Checks

Packed/protected Windows binaries (Enigma, Themida, VMProtect, etc.) validate
the heap handle returned by `GetProcessHeap` / `HeapCreate` to detect
emulators and sandboxes. This document catalogs the checks observed in
`test/exe64win_enigma.bin` (Win11 x64, ntdll build 26100) and the current
state of mwemu's emulation.

Reference sample: `exe64win_enigma.bin`
ntdll version: Windows 11 26100.7920 (x64)
Heap base in mwemu: `0x520000` (hardcoded in PEB64)

> **Status (2026-09-28):** the heap-based anti-emulation described here is
> effectively bypassed. `exe64win_enigma.bin` now emulates to
> **~231,068,733 instructions** (well past all the heap checks below) and
> stops later on an unrelated fault (`set_rip` to non-mapped `0x0`, an
> unresolved API/OEP jump), not on any `_HEAP` check. `LSL` (Blocker 1) is
> implemented; the `BlocksIndex` CFG path (Blocker 2) is no longer on this
> sample's hit path thanks to `VirtualMemoryThreshold = 0xFE00`. See issue
> #196.

---

## 1. Handle value itself

**What real Windows does:** `GetProcessHeap()` returns the base address of
the `_HEAP` structure, a real mapped RW pointer (e.g. `0x0000020A00520000`).
`HeapCreate` also returns the arena base.

**What emulators get wrong:** returning an opaque small integer (slab index
like `0`, `1`, `3`) instead of a mapped address.

**Check observed:**
```
test  rcx, rcx          ; rcx = GetProcessHeap() return value
je    <error_path>       ; NULL → detected
```

Binaries also dereference the handle itself, which faults on an unmapped
small integer.

**Status:** Fixed — mwemu now returns the O1Heap arena base address.

---

## 2. SegmentSignature (`_HEAP+0x10`)

```
cmp  dword ptr [rdi+10h], 0DDEEDDEEh
jne  <error_path>
```

ntdll checks that offset `+0x10` contains the magic value `0xDDEEDDEE`
(the Segment Signature). If it doesn't match, `RtlAllocateHeap` bails out
immediately.

**Status:** Fixed — mwemu writes `0xDDEEDDEE` at init.

---

## 3. Flags / ForceFlags (`_HEAP+0x14`)

```
lea   r8, [rdi+14h]             ; &_HEAP.Flags
mov   gs:[1858h], r8            ; store in TEB.CurrentTransactionHandle (?)
mov   r13d, [r8]                ; read Flags
and   r13d, 2FFAh
test  r13d, 0FFFFFFFDh
jne   <slow_path>
```

ntdll reads the 32-bit Flags word at `+0x14` and masks it. If specific bits
are set (e.g. `HEAP_NO_SERIALIZE` is cleared, validation bits, debug flags),
it takes a different code path. Debugger-attached heaps typically have
`HEAP_TAIL_CHECKING_ENABLED | HEAP_FREE_CHECKING_ENABLED` set, which some
packers use to detect debuggers.

**Status:** Zero is acceptable (normal non-debug heap). No action needed
unless the guest expects specific flag patterns.

---

## 4. FrontEndHeapType / VirtualMemoryThreshold (`_HEAP+0x384`)

```
movzx eax, word ptr [rdi+384h]
cmp   rbx, rax                  ; compare alloc size vs threshold
jae   <large_alloc_path>
```

A 16-bit field at `+0x384` is the `VirtualMemoryThreshold` (or
`FrontEndHeapType` depending on version). If zero, any allocation size ≥ 0
takes the large-allocation path. On real Windows this is typically `0xFF00`
or `0xFE00`.

**Status:** Set to `0xFE00` (realistic Windows default). When this was
zero, all allocations took the large-block path which hit the BlocksIndex
CFG crash (§5). With `0xFE00`, allocations below that threshold use the
segment allocator path instead, which goes further but currently hits an
unimplemented `LSL` (Load Segment Limit) instruction in ntdll at
`0x18016609c`.

---

## 5. BlocksIndex (`_HEAP+0x2C0`)

This is the critical crash point. At `+0x2C0` there is a structure (or
function table pointer) used by ntdll's segment heap allocator:

```
lea   rcx, [rdi+2C0h]           ; pass &BlocksIndex to subroutine
call  ntdll!RtlpHpVsContextAllocateInternal  ; or similar
```

Inside that subroutine, several sub-fields are read:

| Offset from heap base | Size | Field (approx) | Read at |
|---|---|---|---|
| `+0x2C0` | WORD | `BlocksIndex.data[0]` | `movzx ecx, word [rcx]` |
| `+0x2C2` | BYTE | `BlocksIndex.data[2]` | `movzx eax, byte [rcx+2]` |
| `+0x2C4` | BYTE | `BlocksIndex.data[4]` | `movzx eax, byte [rcx+4]` |
| `+0x2C5` | BYTE | `BlocksIndex.data[5]` | `movzx edx, byte [rcx+5]` |
| `+0x2C8` | QWORD | `BlocksIndex.Encoded` | `xor rcx, [rsi+8]` |
| `+0x2D0` | QWORD | `BlocksIndex.ptr1` | various |
| `+0x2D8` | BYTE | `BlocksIndex.flags` | `movzx eax, byte [rcx+18h]` |

Later, ntdll calls `__guard_dispatch_icall_fptr` (`jmp rax`) where RAX was
loaded from the `+0x2C0` region. Since the region is zeroed, the CPU slides
through `add [rax],al` (opcode `00 00`) from `0x5202C0` forward until it
hits `0x5203D8`, where an `add [rdx+rdx*2], al` tries to dereference
`rdx=0x20000*3 = 0x60000` (unmapped) and crashes.

```
ntdll!__guard_dispatch_icall_fptr:
  jmp  rax                        ; rax = 0x5202C0 (zeroed _HEAP memory)
  → slides through 00 00 00 00... (add [rax],al) for 0x118 bytes
  → 0x5203D8: add [rdx+rdx*2],al → deref 0x60000 → crash
```

This is a **CFG indirect call into the heap structure** — ntdll treats part
of the `_HEAP` as a function pointer (likely `CommitRoutine`,
`FrontEndHeap`, or `BlocksIndex` vtable entry). On real Windows it points
to `ntdll!RtlpHpVs*` functions.

**Status:** NOT emulated. This requires populating function pointers inside
the `_HEAP` structure that point to real ntdll code, which is
version-dependent.

---

## 6. SegmentList (`_HEAP+0x138`)

```
mov  rdx, [rdi+138h]    ; SegmentList.Flink
mov  ecx, [rdx+8]       ; → crash if Flink == NULL
```

ntdll walks the `SegmentList` (a `LIST_ENTRY` at `+0x138`). If it's NULL,
the dereference at `[rdx+8]` faults on `0x8`.

**Status:** Fixed — mwemu writes a self-referential sentinel (Flink=Blink=
&SegmentList) so the walk terminates immediately.

---

## 7. FreeLists / LockVariable (`_HEAP+0x3D8`, `_HEAP+0x480`)

```
[rsi + rcx*8 + 80h]     ; resolves to _HEAP+0x3D8 for some rcx
mov  rax, [rdi+480h]     ; LockVariable
```

ntdll reads pointers from these offsets. If NULL, it dereferences NULL+offset
and crashes. These are used for the free-list buckets and the heap's critical
section lock.

**Status:** Fixed — mwemu points them to intra-heap addresses
(`+0x400`, `+0x500`).

---

## 8. First HeapAlloc must not overlap the header

The O1Heap allocator starts handing out memory from offset 0 of the arena.
On real Windows, the first ~0x600 bytes are the `_HEAP` structure itself and
are never returned by `HeapAlloc`. If the allocator returns `0x520000`
(== heap base), the first allocation overwrites all the `_HEAP` struct fields
we populated.

**Status:** Fixed — mwemu reserves the first `0x800` bytes of the process
heap arena via a dummy allocation at init time.

---

## Summary table

| Offset | Size | Field | Needed | Status |
|--------|------|-------|--------|--------|
| `+0x00` | — | Handle value = base addr | Yes | DONE |
| `+0x10` | DWORD | SegmentSignature `0xDDEEDDEE` | Yes | DONE |
| `+0x14` | DWORD | Flags | No (0 ok) | OK |
| `+0x138` | 2×QWORD | SegmentList (LIST_ENTRY) | Yes | DONE |
| `+0x2C0` | ~0x20 | BlocksIndex / FrontEndHeap | Yes | TODO |
| `+0x384` | WORD | VirtualMemoryThreshold | Yes | DONE (`0xFE00`) |
| `+0x3D8` | QWORD | FreeLists ptr | Yes | DONE |
| `+0x480` | QWORD | LockVariable | Yes | DONE |
| `+0x800` | — | Header reservation | Yes | DONE |

---

## _HEAP structure definition

The `Heap64` struct in `crates/libmwemu/src/windows/structures/heap64.rs`
models the fields documented above. It follows the same load/save pattern
as PEB64, TEB64 etc — every field is read/written individually through
`Mem64` accessors (safe Rust, no raw pointer arithmetic).

---

## Next steps to go further with Enigma

### Blocker 1: `LSL` instruction — RESOLVED

With `VirtualMemoryThreshold = 0xFE00`, ntdll takes the segment allocator
path which uses the `LSL` (Load Segment Limit, opcode `0F 03`) instruction
at `0x18016609c`. This is an x86 system instruction that queries the GDT
segment limit for a selector. ntdll uses it to check if the current thread
is running inside a fiber (`IsThreadAFiber` equivalent).

**Status:** Fixed — `LSL` is implemented in the x86 engine
(`engine/instructions/lsl.rs`, dispatched at `engine/mod.rs`). The Enigma
sample no longer crashes here.

### Blocker 2: BlocksIndex at `+0x2C0` (large-alloc path) — not on the current hit path

If `VirtualMemoryThreshold` is 0 (or the allocation exceeds it), ntdll
takes the large-block path which uses CFG-validated indirect calls through
function pointers at `+0x2C0`. Options:

1. **Intercept the ntdll heap functions** (`RtlAllocateHeap` et al) at the
   WinAPI level before they walk the `_HEAP` struct — this is what mwemu
   already does for `kernel32!HeapAlloc`. The problem is the Enigma binary
   resolves the ntdll address via `GetProcAddress` and calls it directly,
   bypassing the kernel32 forwarder that mwemu intercepts.

2. **Populate the function pointers** with addresses of stub routines or
   real ntdll functions. This is fragile and version-dependent.

3. **Hook the CFG dispatch** (`__guard_dispatch_icall_fptr`) to redirect
   calls into the heap region back to mwemu's own allocation engine.
