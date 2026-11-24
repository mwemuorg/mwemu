# GDB integration

mwemu uses `gdbstub` 0.7.10 and `gdbstub_arch` 0.3.3. The crates own RSP framing,
checksums, acknowledgments, command parsing, negotiation, and transfer handling.
mwemu supplies emulator state through their target extension traits. The
`paranoid_unsafe` feature selects gdbstub's safe-code implementation.

## Commands

Start a listener with `gdb 1234` in the mwemu console, then connect:

```text
(gdb) target remote 127.0.0.1:1234
(gdb) info mem
(gdb) monitor maps
(gdb) monitor threads
(gdb) monitor api log on
(gdb) continue
(gdb) monitor api log
(gdb) monitor api log off
(gdb) monitor help
```

Interrupt a running guest before issuing monitor commands. The listener binds
to localhost. The `r2` console command also provides live radare2 access.

- `info mem` uses the standard `MemoryMap` extension on x86, x86-64, and AArch64.
  Readable writable regions are RAM; readable non-writable regions are ROM.
  This format does not describe OS rwx permissions or file paths, and omits
  unreadable regions. Actual memory accesses still enforce mwemu permissions.
- `monitor maps` includes region names and rwx permissions.
- `monitor threads` shows actual emulator thread IDs, PCs, and scheduler states;
  `*` marks the selected thread. GDB register/execution operations still use the
  single-thread target interface.
- `monitor api log` reads the existing bounded resolved-call log. The `on` and
  `off` forms control `cfg.trace_calls`; they do not clear existing entries.
- Normal live memory reads return the entire requested range or an error.
  Single steps execute in the event loop and inspect execution failure and
  memory events before reporting completion.

## Crate choice and upstream work

Keep `gdbstub`: its existing trait-based API already covers memory maps, monitor
commands, threads, reverse execution, section offsets, and extended mode.
Switching crates would not supply mwemu's missing execution and OS semantics.
`rust-gdb-remote-protocol` describes itself as work in progress; the
`ckb-gdb-remote-protocol` fork requires multiprocess and non-stop behavior, which
would add integration work to this currently all-stop target.

The following are candidates for upstream contributions, based on the installed
versions. Confirm them against upstream HEAD and add regressions before opening
PRs; no upstream PR has been created by this change.

| Candidate | Upstream change | Local consequence |
| --- | --- | --- |
| TCP polling | Review `TcpStream::peek` EOF and blocking-mode handling. In 0.7.10, zero-length peeks become `Some(0)`, and peeking leaves the stream nonblocking. | mwemu consumes a pending byte to surface EOF and restores blocking mode before gdbstub writes a reply. A corrected upstream implementation could simplify this adapter. |
| Runtime console output | Provide a public running-state API for guest output, using gdbstub's own RSP encoder. The current public `ConsoleOutput` constructor is limited to crate internals and exposed through monitor callbacks. | Guest stdout/stderr still need forwarding while execution runs. Monitor output already uses `ConsoleOutput`; hand-encoding `O` packets locally would duplicate protocol code. |
| LLDB memory regions | Add `qMemoryRegionInfo` support to the extension API. It is separate from `MemoryMap`, and no corresponding extension exists in the inspected version. | Keep the RAM/ROM XML format standard; richer permissions and names belong in a suitable protocol extension. |
| Architecture descriptions | Complete the x86 SSE target XML and offer AArch64 core/FPU descriptions without advertising every system register. | mwemu currently carries x86 XML overrides. AArch64 defaults advertise system registers the emulator does not implement. Upstream descriptions could remove local XML maintenance. |

## Remaining emulator work

These features have crate APIs already; implementing their target semantics is
mwemu work rather than a reason to replace the protocol library:

- Complete individual register access so `p/P` agrees with `g/G`, including
  floating-point/vector state, and supply accurate ABI/TLS metadata.
- Honor watchpoint ranges, access direction, and overlapping accesses; collect
  memory events when watchpoints are armed. Software breakpoint insertion and
  removal are now idempotent; watchpoints still need the same treatment.
- Replace the generic SIGTERM result with accurate faults, exits, and exit
  status, and define signal delivery, kill, detach, and reconnect behavior.
- Implement `MultiThreadBase`/resume operations against actual thread contexts
  and the scheduler. See [multithreading](MULTITHREADING.md).
- Generate library information from loader metadata across PE, ELF, and Mach-O;
  the current suffix filter and fixed segment offset are incomplete.
- Supply real relocation metadata before enabling `SectionOffsets`/`qOffsets`.
  `emu.base` alone is not necessarily a section relocation offset.
- Add extended launch/attach, arguments, environment, and cwd with explicit
  guest lifecycle semantics.
- Add reverse execution using deterministic history/replay and complete
  checkpoints. [mwemu-snapshot](../crates/mwemu-snapshot/README.md) captures a live
  Linux process; it is not an instruction-history engine. Restoring one snapshot
  does not implement reverse-step or reverse-continue.

## Validation status

Tests, builds, and clippy were deliberately deferred at the user's request
until the radare2 GDB work is ready. Only formatting and source/diff review were
performed. The existing live-memory regression's short-read expectation was
updated to require an error, but the test was not run.

Use [TEST.md](../TEST.md) when joint validation starts. Include partial TCP
frames, incoming packets while running, EOF and Ctrl-C, memory gaps, step
failures/watch events, chunked memory-map transfers, monitor output, and repeated
software-breakpoint operations on all three guest architectures.

## References

- [gdbstub project and supported features](https://github.com/daniel5151/gdbstub)
- [gdbstub extension API](https://docs.rs/gdbstub/0.7.10/gdbstub/target/ext/index.html)
- [GDB memory-map format](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Memory-Map-Format.html)
- [GDB query and monitor packets](https://sourceware.org/gdb/current/onlinedocs/gdb.html/General-Query-Packets.html)
- [GDB stop replies and console output](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Stop-Reply-Packets.html)
- [rust-gdb-remote-protocol](https://github.com/luser/rust-gdb-remote-protocol)
- [ckb-gdb-remote-protocol API](https://docs.rs/ckb-gdb-remote-protocol/latest/ckb_gdb_remote_protocol/)
