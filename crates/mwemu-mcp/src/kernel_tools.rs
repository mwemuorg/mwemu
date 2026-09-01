//! Kernel-mode (driver) tools.
//!
//! These expose libmwemu's driver emulation: link a `.ko` against the emulated
//! kernel, drive its entry points, and read back the allocator ledger and the
//! memory-safety findings. The workflow an agent follows is:
//!
//! 1. `mwemu_open` (x64) then `mwemu_kernel_load_module`
//! 2. `mwemu_kernel_init` — run the module's init, as `insmod` would
//! 3. `mwemu_alloc` + `mwemu_write_int` to build an ioctl argument struct
//! 4. `mwemu_kernel_call` for each handler you want to exercise
//! 5. `mwemu_kernel_findings` — what the run proved
//!
//! Every call that runs guest code returns the findings it produced, so a bug
//! is attributed to the exact step that triggered it rather than to the run as
//! a whole.

use serde_json::{Value, json};

use libmwemu::kernel::heap::ChunkState;

use crate::server::Server;
use crate::util::*;

fn hx(n: u64) -> String {
    format!("0x{n:x}")
}

/// Render one finding, including the provenance an analyst needs to act on it.
fn finding_json(f: &libmwemu::kernel::guard::Finding) -> Value {
    let o = &f.origin;
    json!({
        "kind": f.kind.tag(),
        "title": f.kind.label(),
        "address": hx(f.addr),
        "access_size": f.size,
        "faulting_pc": hx(f.rip),
        "step": f.pos,
        "hits": f.hits,
        "object": if o.addr == 0 { Value::Null } else { json!({
            "address": hx(o.addr),
            "cache": o.cache,
            "requested_size": o.req_size,
            "bucket_size": o.size,
            "offset": f.addr.wrapping_sub(o.addr),
            "allocated_by": o.alloc_api,
            "allocated_at": hx(o.alloc_rip),
            "allocated_step": o.alloc_pos,
            "freed_by": if o.free_api.is_empty() { Value::Null } else { json!(o.free_api) },
            "freed_at": hx(o.free_rip),
            "freed_step": o.free_pos,
        })},
        "report": f.report(),
    })
}

/// Findings added since `from`, so each step reports only what it caused.
fn findings_since(s: &Server, from: usize) -> Value {
    match s.emu.as_ref() {
        Some(e) => Value::Array(
            e.kernel_findings()
                .iter()
                .skip(from)
                .map(finding_json)
                .collect(),
        ),
        None => Value::Array(vec![]),
    }
}

fn finding_count(s: &Server) -> usize {
    s.emu
        .as_ref()
        .map(|e| e.kernel_findings().len())
        .unwrap_or(0)
}

// --- handlers ----------------------------------------------------------------

pub fn t_load_module(s: &mut Server, a: &Value) -> Result<Value, String> {
    s.require_disk()?;
    let path = req_str(a, "path")?.to_string();
    let emu = s.emu_mut()?;
    emu.load_kernel_module(&path)
        .map_err(|e| format!("cannot load kernel module: {e}"))?;

    let kernel = emu.kernel.as_ref().expect("kernel env after load");
    let m = &kernel.module;
    Ok(json!({
        "ok": true,
        "name": m.name,
        "base": hx(m.base),
        "size": m.size,
        "init": m.init.map(hx),
        "exit": m.exit.map(hx),
        "sections": m.sections.iter().map(|sec| json!({
            "name": sec.name,
            "address": hx(sec.addr),
            "size": sec.size,
            "exec": sec.perm.execute,
            "write": sec.perm.write,
        })).collect::<Vec<_>>(),
        "symbol_count": m.symbols.len(),
        "imports": kernel.stub_by_name.len() + kernel.data_by_name.len(),
        "unresolved": m.unresolved,
    }))
}

pub fn t_init(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let before = finding_count(s);
    let emu = s.emu_mut()?;
    let ret = emu
        .run_module_init()
        .map_err(|e| format!("module init failed: {e}"))?;
    Ok(json!({
        "returned": ret as i64,
        "success": ret == 0,
        "findings": findings_since(s, before),
    }))
}

pub fn t_exit(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let before = finding_count(s);
    let emu = s.emu_mut()?;
    let ret = emu
        .run_module_exit()
        .map_err(|e| format!("module exit failed: {e}"))?;
    Ok(json!({
        "returned": ret as i64,
        "findings": findings_since(s, before),
    }))
}

pub fn t_call(s: &mut Server, a: &Value) -> Result<Value, String> {
    let mut args: Vec<u64> = Vec::new();
    if let Some(list) = a.get("args").and_then(|v| v.as_array()) {
        for v in list {
            args.push(parse_u64(v)?);
        }
    }

    let before = finding_count(s);
    let emu = s.emu_mut()?;
    let (target, label) = match opt_str(a, "symbol") {
        Some(sym) => (
            emu.module_symbol(sym)
                .ok_or_else(|| format!("module has no symbol '{sym}'"))?,
            sym.to_string(),
        ),
        None => {
            let addr = req_u64(a, "address")?;
            (addr, hx(addr))
        }
    };

    // A driver that faults is the normal outcome when a bug fires, so a failed
    // run is reported, not raised: the findings are the answer either way.
    let (ret, error) = match emu.kernel_call(target, &args) {
        Ok(v) => (Some(v as i64), Value::Null),
        Err(e) => (None, json!(e.to_string())),
    };

    Ok(json!({
        "called": label,
        "address": hx(target),
        "returned": ret,
        "error": error,
        "findings": findings_since(s, before),
    }))
}

pub fn t_symbols(s: &mut Server, a: &Value) -> Result<Value, String> {
    let filter = opt_str(a, "filter").unwrap_or("").to_ascii_lowercase();
    let limit = opt_u64(a, "limit")?.unwrap_or(200) as usize;
    let emu = s.emu()?;
    let kernel = emu
        .kernel
        .as_ref()
        .ok_or_else(|| "no kernel module loaded".to_string())?;

    let mut out: Vec<Value> = kernel
        .module
        .symbols
        .iter()
        .filter(|sym| filter.is_empty() || sym.name.to_ascii_lowercase().contains(&filter))
        .map(|sym| {
            json!({
                "name": sym.name,
                "address": hx(sym.addr),
                "size": sym.size,
                "kind": if sym.is_func { "function" } else { "object" },
                "global": sym.is_global,
            })
        })
        .collect();
    let total = out.len();
    out.truncate(limit);

    Ok(json!({ "total": total, "returned": out.len(), "symbols": out }))
}

pub fn t_findings(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let emu = s.emu()?;
    let findings = emu.kernel_findings();
    Ok(json!({
        "count": findings.len(),
        "use_after_free": emu.kernel_found_uaf(),
        "findings": findings.iter().map(finding_json).collect::<Vec<_>>(),
    }))
}

pub fn t_heap(s: &mut Server, a: &Value) -> Result<Value, String> {
    let want = opt_str(a, "state").unwrap_or("all").to_ascii_lowercase();
    let emu = s.emu()?;
    let kernel = emu
        .kernel
        .as_ref()
        .ok_or_else(|| "no kernel module loaded".to_string())?;

    let chunks: Vec<Value> = kernel
        .heap
        .chunks()
        .iter()
        .filter(|c| match want.as_str() {
            "live" => c.state == ChunkState::Live,
            "freed" => c.state == ChunkState::Freed,
            _ => true,
        })
        .map(|c| {
            json!({
                "address": hx(c.addr),
                "size": c.size,
                "requested_size": c.req_size,
                "region": c.region.label(),
                "cache": c.cache,
                "state": if c.is_freed() { "freed" } else { "live" },
                "allocated_by": c.alloc_api,
                "allocated_at": hx(c.alloc_rip),
                "allocated_step": c.alloc_pos,
                "freed_by": if c.free_api.is_empty() { Value::Null } else { json!(c.free_api) },
                "freed_at": hx(c.free_rip),
                "freed_step": c.free_pos,
            })
        })
        .collect();

    Ok(json!({
        "count": chunks.len(),
        "live_bytes": kernel.heap.live_bytes(),
        "chunks": chunks,
    }))
}

pub fn t_log(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let emu = s.emu()?;
    let kernel = emu
        .kernel
        .as_ref()
        .ok_or_else(|| "no kernel module loaded".to_string())?;
    Ok(json!({
        "lines": kernel.log,
        "unimplemented_apis": kernel.unimplemented,
    }))
}

pub fn t_run_deferred(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let before = finding_count(s);
    let emu = s.emu_mut()?;
    let ran = emu.kernel_run_deferred();
    Ok(json!({
        "callbacks_run": ran,
        "findings": findings_since(s, before),
    }))
}

pub fn t_leak_check(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let before = finding_count(s);
    let emu = s.emu_mut()?;
    emu.kernel_check_leaks();
    Ok(json!({ "findings": findings_since(s, before) }))
}

pub fn t_surface(s: &mut Server, a: &Value) -> Result<Value, String> {
    let _ = s;
    let os = opt_str(a, "os").unwrap_or("linux").to_ascii_lowercase();
    let surface = match os.as_str() {
        "linux" => libmwemu::kernel::linux::SURFACE,
        "windows" => libmwemu::kernel::windows::SURFACE,
        "macos" | "darwin" => libmwemu::kernel::macos::SURFACE,
        other => {
            return Err(format!(
                "unknown kernel '{other}', use linux, windows or macos"
            ));
        }
    };
    let groups: Vec<Value> = surface
        .iter()
        .map(|(group, names)| json!({ "group": group, "symbols": names }))
        .collect();
    Ok(json!({ "os": os, "groups": groups }))
}

// --- schemas -----------------------------------------------------------------

fn obj(props: Value, required: &[&str]) -> Value {
    json!({
        "type": "object",
        "properties": props,
        "required": required,
        "additionalProperties": false
    })
}

pub fn sc_empty() -> Value {
    json!({ "type": "object", "properties": {}, "additionalProperties": false })
}

pub fn sc_load_module() -> Value {
    obj(
        json!({ "path": { "type": "string", "description": "Path to the .ko file" } }),
        &["path"],
    )
}

pub fn sc_call() -> Value {
    obj(
        json!({
            "symbol": { "type": "string", "description": "Name of a function the module defines, e.g. an ioctl handler" },
            "address": { "type": ["string","integer"], "description": "Call by address instead of by symbol" },
            "args": { "type": "array", "items": { "type": ["string","integer"] }, "description": "Integer arguments, kernel calling convention" }
        }),
        &[],
    )
}

pub fn sc_symbols() -> Value {
    obj(
        json!({
            "filter": { "type": "string", "description": "Case-insensitive substring of the symbol name" },
            "limit": { "type": ["string","integer"], "description": "Maximum symbols to return (default 200)" }
        }),
        &[],
    )
}

pub fn sc_heap() -> Value {
    obj(
        json!({ "state": { "type": "string", "enum": ["all","live","freed"] } }),
        &[],
    )
}

pub fn sc_surface() -> Value {
    obj(
        json!({ "os": { "type": "string", "enum": ["linux","windows","macos"] } }),
        &[],
    )
}
