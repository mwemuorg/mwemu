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
`mwemu_init_win32`, `mwemu_init_linux64`, `mwemu_alloc`, `mwemu_free`.

Memory: `mwemu_read_mem`, `mwemu_read_string`, `mwemu_write_mem`,
`mwemu_write_string`, `mwemu_write_int`, `mwemu_memset`, `mwemu_search`,
`mwemu_maps`.

Registers / stack: `mwemu_get_reg`, `mwemu_set_reg`, `mwemu_regs`,
`mwemu_get_xmm`, `mwemu_set_xmm`, `mwemu_stack_push`, `mwemu_stack_pop`.

Execution: `mwemu_step`, `mwemu_run`, `mwemu_run_to`, `mwemu_run_until_return`,
`mwemu_run_until_apicall`, `mwemu_call`, `mwemu_set_pc`.

Inspect: `mwemu_disassemble`, `mwemu_call_stack`, `mwemu_prev_mnemonic`,
`mwemu_api_addr_to_name`, `mwemu_api_name_to_addr`, `mwemu_bp`.

`*` = disk-gated (sandbox mode disables them).

Addresses and integers may be passed as JSON numbers **or** strings
(`"0x401000"`), since 64-bit addresses don't fit a JSON number exactly.

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
