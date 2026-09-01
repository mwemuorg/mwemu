# mwemu-mcp
[![Rust CI](https://github.com/mwemuorg/mwemu-mcp/actions/workflows/ci.yaml/badge.svg)](https://github.com/mwemuorg/mwemu-mcp/actions/workflows/ci.yaml)

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server that
exposes the [mwemu](https://github.com/mwemuorg/mwemu) x86 / x86-64 / aarch64
emulator (`libmwemu`) to MCP clients such as Claude.

It works like driving `pymwemu` by hand, but over MCP: you **open** a session
for an architecture, **configure** it, **prepare** memory (allocs, register and
memory writes, environment init), then **emulate** step by step and **inspect**
the result — all through discrete tools. It is a *session* over an open binary,
not a one-shot full emulation.

```
open ──▶ configure ──▶ prepare ──▶ emulate ──▶ inspect ──▶ close
 arch    base/stack    alloc        step/run    regs
         limits        write_mem    call        read_mem
                       set_reg      run_to       disassemble
                       load         until_ret    maps
```

## Status / scope (v1)

- **Mono-session**: one emulator open at a time; `mwemu_open` (re)creates it.
- **Offline**: emulated networking is forced off (no real sockets / HTTP).
- **Transports**: `stdio` by default (like radare2-mcp); optional loopback HTTP.
- Synchronous, single-threaded, no async runtime — matching mwemu's design.

## Build

```sh
# stdio only (minimal):
cargo build -p mwemu-mcp --release

# with the optional loopback HTTP transport:
cargo build -p mwemu-mcp --release --features http
```

The binary is `target/release/mwemu-mcp`.

## Run

```sh
mwemu-mcp                       # stdio (default)
mwemu-mcp --http 127.0.0.1:8765 # loopback HTTP (needs --features http)
mwemu-mcp --maps ./maps64       # preset a trusted maps folder
```

| Flag | Meaning |
|------|---------|
| `--http <addr>` | Serve over HTTP instead of stdio. **Loopback only** (`127.0.0.1`/`::1`/`localhost`); non-loopback addresses are refused. |
| `--unsafe` | Allow filesystem-path tools even over the network transport. |
| `--safe` | Force the sandbox on (also on stdio). |
| `--maps <folder>` | Trusted maps folder applied on `mwemu_open`. |
| `--log <level>` | Log level to stderr (`off`..`trace`). Needed to see libmwemu's instruction trace (emitted via `log`, gated by `mwemu_config` `verbose`). Goes to the server's stderr, not the tool replies. |

### Connecting an MCP client

stdio (e.g. an `mcp.json` / Claude Desktop config):

```json
{
  "mcpServers": {
    "mwemu": { "command": "/path/to/mwemu-mcp", "args": [] }
  }
}
```

HTTP: point the client at `http://127.0.0.1:8765` (single JSON-RPC message per
POST, `application/json` reply).

## Security model

mwemu deliberately emulates file and network APIs, and its loaders open files by
path — so an MCP client that could choose paths or run untrusted code could turn
the server into an LFI / arbitrary-write primitive. Defense is layered:

1. **Tool gating.** Path-taking tools (`mwemu_load_binary`, `mwemu_load_maps`,
   the `maps_folder` config field) are **disabled in sandbox mode**. Clients feed
   code as inline bytes (`mwemu_load_code_bytes`); the operator presets any
   trusted paths at startup with `--maps`.
2. **Default posture.** stdio is treated as a local, trusted channel (disk on).
   The HTTP transport is **sandboxed by default** (disk off). `--unsafe` lifts
   it; `--safe` forces it on everywhere.
3. **Offline.** Emulated networking is forced off on every session.
4. **Loopback + DNS-rebind guard.** HTTP binds to loopback only and rejects
   requests whose `Host` header is not local (mitigates a browser pointed at
   `localhost`).
5. **Panic containment.** libmwemu's loaders/decoders can panic on malformed
   input; each tool call is wrapped so a bad input returns an error instead of
   crashing the server.

> **For untrusted samples, add OS-level isolation.** A pure in-process sandbox
> cannot fully contain an emulator that emulates file/network APIs. Run the
> server in a container with `--network none`, a read-only rootfs and a non-root
> user (or equivalent namespaces/seccomp) when analysing hostile code.

## Tools

Lifecycle: `mwemu_open`, `mwemu_close`, `mwemu_status`, `mwemu_config`.

Load / prepare: `mwemu_load_code_bytes`, `mwemu_load_binary`*, `mwemu_load_maps`*,
`mwemu_set_winver`*, `mwemu_init_win32`, `mwemu_init_linux64`, `mwemu_alloc`, `mwemu_free`.

Memory: `mwemu_read_mem`, `mwemu_read_string`, `mwemu_write_mem`,
`mwemu_write_string`, `mwemu_write_int`, `mwemu_memset`, `mwemu_search`,
`mwemu_maps`.

Registers / stack: `mwemu_get_reg`, `mwemu_set_reg`, `mwemu_regs`,
`mwemu_get_xmm`, `mwemu_set_xmm`, `mwemu_stack_push`, `mwemu_stack_pop`.

Execution: `mwemu_step`, `mwemu_run`, `mwemu_run_to`, `mwemu_run_until_return`,
`mwemu_run_until_apicall`, `mwemu_call`, `mwemu_set_pc`.

Inspect: `mwemu_disassemble`, `mwemu_call_stack`, `mwemu_prev_mnemonic`,
`mwemu_api_addr_to_name`, `mwemu_api_name_to_addr`, `mwemu_api_call_trace`, `mwemu_bp`.

Kernel mode (drivers): `mwemu_kernel_load_module`*, `mwemu_kernel_init`,
`mwemu_kernel_exit`, `mwemu_kernel_call`, `mwemu_kernel_symbols`,
`mwemu_kernel_findings`, `mwemu_kernel_heap`, `mwemu_kernel_log`,
`mwemu_kernel_run_deferred`, `mwemu_kernel_leak_check`, `mwemu_kernel_surface`.

`*` = disk-gated (sandbox mode disables them).

Addresses and integers may be passed as JSON numbers **or** strings
(`"0x401000"`), since 64-bit addresses don't fit a JSON number exactly.

### Seeing Windows API names while emulating

mwemu can resolve every call it makes into a Windows API name (from its
built-in stub table by default, or from real PE export tables when genuine
system DLLs are loaded) — this is surfaced two ways over MCP:

- **One at a time**: `mwemu_run_until_apicall` stops at the next API call and
  returns its address and name directly.
- **As a trace**: set `trace_calls: true` via `mwemu_config`, then run/step
  normally (`mwemu_step`, `mwemu_run`, `mwemu_call`, ...) and pull the
  accumulated log with `mwemu_api_call_trace` (`action: "get"`, or `"clear"`
  to reset it). Each entry has `pos`, `from`, `to` and the resolved `name`.

For real export names (not just the synthetic stub table) call
`mwemu_set_winver` — e.g. `{"winver": "win11"}` — right after `mwemu_open` and
before `mwemu_init_win32`. It fetches genuine ntdll/kernelbase/kernel32/...
for the session's architecture from Microsoft's public symbol server
(`msdl.microsoft.com`), caches them on disk, and points the session's maps
folder at them. It's disk-gated like `mwemu_load_maps` (sandboxed over the
network transport unless `--unsafe`).

### Emulating a driver and finding lifetime bugs

A Linux kernel module is not a program: it has no entry point, and every
function it calls belongs to a kernel that is not there. `mwemu_kernel_load_module`
links the `.ko` against an emulated one — sections placed, relocations applied,
every import resolved to an interceptable stub — and models the slab allocator
explicitly. Freed chunks are not recycled; they are poisoned and kept, so a
stale pointer is *observed* rather than silently working.

```
mwemu_open           {"arch": "x64"}
mwemu_kernel_load_module {"path": "driver.ko"}     -> base, sections, imports
mwemu_kernel_init                                  -> runs init_module (insmod)
mwemu_kernel_symbols {"filter": "ioctl"}           -> find the handlers
mwemu_alloc          {"name":"userbuf","size":4096}
mwemu_write_int      ...                            build the ioctl argument
mwemu_kernel_call    {"symbol":"drv_ioctl","args":[0,"0x1002","0x10000000"]}
mwemu_kernel_findings                              -> what the run proved
```

Every call that runs guest code returns the findings it caused, so a bug is
attributed to the step that triggered it. A finding names the faulting
instruction, the object, its slab cache, and both the allocation and the free
site:

```
BUG: KMWEMU: use-after-free (read) in tlm_channel of size 8 at addr 0xffff888000001128
  faulting instruction at 0xffffffffc00007fb (step 478)
  object 0xffff888000001100..0xffff888000001160 (requested 88 bytes, bucket 96), offset 40
  allocated by kmem_cache_alloc_noprof at 0xffffffffc000056c (step 86)
  freed by kmem_cache_free at 0xffffffffc00004e9 (step 407)
```

`mwemu_kernel_surface` lists the kernel API that is implemented, grouped by
subsystem, so you can tell in advance whether a given driver will run.

## Example (raw JSON-RPC over stdio)

```jsonc
// open a 64-bit session
{"jsonrpc":"2.0","id":1,"method":"tools/call",
 "params":{"name":"mwemu_open","arguments":{"arch":"x64"}}}
// load `mov rax, 0x3039 ; inc rax`
{"jsonrpc":"2.0","id":2,"method":"tools/call",
 "params":{"name":"mwemu_load_code_bytes","arguments":{"hex":"48c7c039300000 48ffc0"}}}
// step two instructions
{"jsonrpc":"2.0","id":3,"method":"tools/call",
 "params":{"name":"mwemu_step","arguments":{"count":2}}}
// read rax  ->  0x303a
{"jsonrpc":"2.0","id":4,"method":"tools/call",
 "params":{"name":"mwemu_get_reg","arguments":{"reg":"rax"}}}
```

## License

MIT, same as mwemu.
