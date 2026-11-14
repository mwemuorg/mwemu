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

// --- Windows drivers -----------------------------------------------------------

fn privop_json(p: &libmwemu::kernel::PrivilegedOp) -> Value {
    json!({
        "kind": p.kind,
        "address": hx(p.addr),
        "value": hx(p.value),
        "size": p.size,
        "pc": hx(p.rip),
        "step": p.pos,
        "report": p.report(),
    })
}

fn irp_result_json(r: &libmwemu::kernel::windows::irp::IrpResult) -> Value {
    json!({
        "status": format!("0x{:08x}", r.status),
        "io_status": format!("0x{:08x}", r.io_status),
        "information": r.information,
        "completed": r.completed,
        "succeeded": r.succeeded(),
        "error": r.error,
        "output_hex": r.output.iter().map(|b| format!("{b:02x}")).collect::<String>(),
        "privileged_ops": r.privops.iter().map(privop_json).collect::<Vec<_>>(),
        "unimplemented_apis": r.unimplemented,
    })
}

/// What a Windows driver exposed after DriverEntry: devices, symbolic links
/// and the dispatch routines it installed.
pub fn t_driver(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let emu = s.emu()?;
    let kernel = emu
        .kernel
        .as_ref()
        .ok_or_else(|| "no kernel module loaded".to_string())?;
    Ok(json!({
        "driver_object": hx(emu.kernel_driver_object()),
        "devices": emu.kernel_devices().iter().map(|d| json!({
            "name": d.name,
            "address": hx(d.addr),
            "device_type": format!("0x{:x}", d.device_type),
            "extension_size": d.extension_size,
        })).collect::<Vec<_>>(),
        "symlinks": emu.kernel_symlinks().iter().map(|(l, t)| json!({ "link": l, "target": t })).collect::<Vec<_>>(),
        "dispatch": emu.kernel_dispatch_table().iter().map(|(i, name, addr)| json!({
            "major": i,
            "name": name,
            "address": hx(*addr),
        })).collect::<Vec<_>>(),
        "unload": kernel.module.exit.map(hx),
        "unresolved_imports": kernel.module.unresolved,
        "callbacks": kernel.win.callbacks.iter().map(|c| json!({
            "api": c.api, "function": hx(c.func), "context": hx(c.context),
        })).collect::<Vec<_>>(),
    }))
}

/// Send one IOCTL to a device the driver created.
pub fn t_ioctl(s: &mut Server, a: &Value) -> Result<Value, String> {
    let code = req_u64(a, "code")? as u32;
    let device = opt_u64(a, "device")?.unwrap_or(0);
    let input = if a.get("hex").is_some() || a.get("bytes").is_some() {
        get_bytes(a)?
    } else {
        Vec::new()
    };
    let output_len = opt_u64(a, "output_len")?.unwrap_or(0x100) as usize;
    let before = finding_count(s);
    let emu = s.emu_mut()?;
    let r = emu
        .kernel_ioctl(device, code, &input, output_len)
        .map_err(|e| format!("cannot deliver the IOCTL: {e}"))?;
    let mut v = irp_result_json(&r);
    v["code"] = json!(format!("0x{:08x}", code));
    v["findings"] = findings_since(s, before);
    Ok(v)
}

/// Send a bufferless request: IRP_MJ_CREATE (0), IRP_MJ_CLOSE (2), IRP_MJ_CLEANUP (0x12), ...
pub fn t_send_irp(s: &mut Server, a: &Value) -> Result<Value, String> {
    let major = req_u64(a, "major")? as u8;
    let minor = opt_u64(a, "minor")?.unwrap_or(0) as u8;
    let device = opt_u64(a, "device")?.unwrap_or(0);
    let before = finding_count(s);
    let emu = s.emu_mut()?;
    let r = emu
        .kernel_send_irp(device, major, minor)
        .map_err(|e| format!("cannot deliver the request: {e}"))?;
    let mut v = irp_result_json(&r);
    v["findings"] = findings_since(s, before);
    Ok(v)
}

/// Every privileged operation recorded so far (MSR, port I/O, physical memory).
pub fn t_privops(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let emu = s.emu()?;
    let ops = emu.kernel_privops();
    Ok(json!({
        "count": ops.len(),
        "operations": ops.iter().map(privop_json).collect::<Vec<_>>(),
    }))
}

/// Harvest the IOCTL codes from the driver's code and send each one.
pub fn t_scan_ioctls(s: &mut Server, a: &Value) -> Result<Value, String> {
    let device = opt_u64(a, "device")?.unwrap_or(0);
    let output_len = opt_u64(a, "output_len")?.unwrap_or(0x1000) as usize;
    let emu = s.emu_mut()?;
    let candidates = emu.kernel_ioctl_candidates();
    let probes = emu.kernel_scan_ioctls(device, output_len);
    Ok(json!({
        "candidates": candidates.iter().map(|c| format!("0x{:08x}", c)).collect::<Vec<_>>(),
        "probes": probes.iter().map(|p| {
            let mut v = irp_result_json(&p.result);
            v["code"] = json!(format!("0x{:08x}", p.code));
            v["input_len"] = json!(p.input_len);
            v["report"] = json!(p.report());
            v
        }).collect::<Vec<_>>(),
    }))
}

/// Preset an MSR, an I/O port or a PCI config byte the driver will read.
pub fn t_preset(s: &mut Server, a: &Value) -> Result<Value, String> {
    let value = req_u64(a, "value")?;
    let emu = s.emu_mut()?;
    if let Some(msr) = opt_u64(a, "msr")? {
        emu.kernel_set_msr(msr as u32, value);
    } else if let Some(port) = opt_u64(a, "port")? {
        emu.kernel_set_port(port as u16, value);
    } else if let Some(addr) = opt_u64(a, "register")? {
        emu.kernel_set_register(addr, value);
    } else {
        return Err("provide 'msr', 'port' or 'register'".to_string());
    }
    Ok(json!({ "ok": true }))
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
        json!({ "path": { "type": "string", "description": "Path to the driver: a Linux .ko, a Windows .sys or a macOS kext" } }),
        &["path"],
    )
}

pub fn sc_ioctl() -> Value {
    obj(
        json!({
            "code": { "type": ["string","integer"], "description": "IOCTL control code, e.g. \"0x9c402118\"" },
            "hex": { "type": "string", "description": "Input buffer as hex" },
            "bytes": { "type": "array", "items": { "type": "integer" }, "description": "Input buffer as bytes" },
            "output_len": { "type": "integer", "description": "Size of the caller's output buffer (default 256)" },
            "device": { "type": ["string","integer"], "description": "DEVICE_OBJECT address; default: the first device the driver created" }
        }),
        &["code"],
    )
}

pub fn sc_send_irp() -> Value {
    obj(
        json!({
            "major": { "type": "integer", "description": "IRP_MJ_* index: 0 create, 2 close, 0x12 cleanup, 0x10 shutdown" },
            "minor": { "type": "integer", "description": "Minor function (default 0)" },
            "device": { "type": ["string","integer"], "description": "DEVICE_OBJECT address; default: the first device" }
        }),
        &["major"],
    )
}

pub fn sc_scan_ioctls() -> Value {
    obj(
        json!({
            "device": { "type": ["string","integer"], "description": "DEVICE_OBJECT address; default: the first device" },
            "output_len": { "type": "integer", "description": "Output buffer size for every probe (default 4096)" }
        }),
        &[],
    )
}

pub fn sc_preset() -> Value {
    obj(
        json!({
            "msr": { "type": ["string","integer"], "description": "MSR index to preset" },
            "port": { "type": ["string","integer"], "description": "I/O port to preset" },
            "register": { "type": ["string","integer"], "description": "Device register / PCI config key to preset" },
            "value": { "type": ["string","integer"], "description": "Value the driver will read" }
        }),
        &["value"],
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
