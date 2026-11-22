# mwemu-snapshot

Clone a **live Linux process** into [mwemu](../mwemu): ptrace-attach (or spawn
a target ourselves), copy its registers and its entire readable memory map,
and hand [libmwemu](../libmwemu) a real `Emu` at exactly the point it was
captured — no relocation, no `ld.so` emulation needed, since the real dynamic
linker already did that work before we took the snapshot.

All the `unsafe`/ptrace code lives here, in this standalone binary — never
inside `libmwemu` itself, which stays free of it. The only change this
required in `libmwemu` was exposing `serialization::emu`/`serialization::maps`
as `pub` so an external crate can build a `SerializableEmu` directly.

## Usage

```sh
# Spawn a target ourselves and snapshot it immediately (no privilege needed —
# it's our own child, so Yama's default ptrace_scope allows it).
mwemu-snapshot spawn <command> [args...] [-o out.mwemu]

# Attach to an already-running, unrelated PID. Needs CAP_SYS_PTRACE/root, or
# the target having called prctl(PR_SET_PTRACER, ..) — Yama's default
# ptrace_scope=1 only allows a direct child otherwise (same restriction gdb
# would hit).
mwemu-snapshot attach <pid> [-o out.mwemu]
```

The snapshot is saved in mwemu's own native format (not minidump — see
"Why not minidump" below), and reopens with the `mwemu` CLI:

```sh
mwemu -f <any file of the same arch, its content is ignored> -d out.mwemu -v
```

(`-f` is currently required to bootstrap the CLI's config before `-d`
overrides the whole `Emu` state with the snapshot.)

Use a matching mwemu build to load a capture in the development format. See [native snapshot compatibility](../../docs/serialization-roadmap.md#native-snapshot-compatibility).

## Why not minidump

The natural first idea — ptrace via [`minidump-writer`](https://github.com/rust-minidump/minidump-writer)
producing a real `.dmp`, read by mwemu's existing (Windows-oriented)
`MinidumpReader` — turned out to be a dead end as shipped: `minidump-writer`
0.13.0's `write_memory_info_list_stream` uses `procfs_core` 0.18 to parse
`/proc/pid/maps`, which fails on this kernel, and that failure is hard-coded
as fatal to the whole dump (`?`, no way to skip the section from the public
API) — even though it's metadata mwemu's own reader doesn't strictly need.
Rather than fork/patch a third-party crate for a section we don't use, this
tool talks to `/proc/pid/maps` and `ptrace`/`process_vm_readv` directly (via
[`nix`](https://docs.rs/nix)) and builds the `SerializableEmu` straight from
that — no file-format round-trip, no unrelated dependency bug.

## Known gap: FPU/SSE/AVX state isn't captured yet

Only general-purpose registers are captured (`PTRACE_GETREGS`), not FPU/SSE/AVX
(`PTRACE_GETFPREGS` / `PTRACE_GETREGSET` with `NT_X86_XSTATE`). This is
concretely observable: `mwemu-snapshot spawn sleep 300` resumes and replays the
real `ld.so`/libc startup faithfully — real syscalls, real addresses, all the
way into `sleep`'s own `main()` — but GNU `sleep`'s fractional-seconds parser
(`xstrtod`, uses SSE) then rejects `"300"` as an invalid interval, because the
emulator starts with a zeroed FPU instead of the process's real one. Next step.

## Other known limits (by design, for now)

- **Single-threaded targets only.** Each Linux thread is independently
  ptrace-attachable via `/proc/pid/task/<tid>`, so multi-thread *capture* is
  mechanically simple to add; the real cost is *resuming* several threads
  with faithful concurrency semantics against mwemu's syscall layer — a scope
  decision, not a blocker, deferred for now.
- **Open file descriptors aren't captured.** A regular file is recoverable
  (`/proc/pid/fd` + `/proc/pid/fdinfo` give path + offset — reopen and seek);
  a live socket/pipe/epoll fd fundamentally isn't clonable this way.
- **`[vvar]`/`[vvar_vclock]`/`[vdso]`/`[vsyscall]` are left unmapped**, and so
  is any GPU-driver-backed mapping (seen in practice: dozens of
  `anon_inode:i915.gem` regions on a Telegram Desktop capture — Intel DRM/i915
  GEM buffer objects). `process_vm_readv` genuinely can't reach any of these
  cross-process (`EFAULT`) — they're `VM_PFNMAP`-style special mappings with
  no backing `struct page`, not ordinary anon/file memory.
  A same-kernel substitute for the `vvar`/`vdso` family was tried and reverted
  — worth recording so it isn't retried blind: `/proc/self/mem` can't reach
  them either (same underlying `get_user_pages` limitation, even reading our
  *own* copy), and a raw pointer load crashed this tool with **SIGBUS** on
  `[vvar]` itself despite `/proc/pid/maps` reporting it `r--p` — the reported
  permission bit does not reliably predict whether a plain CPU load is safe on
  these pages, and a live crash on the analyst's own machine is a worse
  outcome than an absent region. GPU buffer content can't be substituted this
  way at all regardless — it's live, context-specific rendering data.
  If `[vvar]`/`[vdso]` support is wanted later, the safe place for it is the
  *replay* side inside mwemu itself (a sandboxed, catchable context)
  synthesizing its own page when it recognizes the access pattern — not a live
  host-side read here.
