//! Tool registry and handlers.
//!
//! Each tool mirrors a libmwemu / pymwemu capability, organised around the
//! session lifecycle: open -> configure -> prepare -> emulate -> inspect ->
//! close. Handlers operate on the single open `Emu` held by the `Server`.

use serde_json::{json, Value};

use libmwemu::maps::mem64::Permission;
use libmwemu::{emu32, emu64, emu_aarch64};

use crate::server::Server;
use crate::util::*;

/// One MCP tool: name, human description, JSON-Schema builder and handler.
pub struct ToolDef {
    pub name: &'static str,
    pub description: &'static str,
    pub schema: fn() -> Value,
    pub handler: fn(&mut Server, &Value) -> Result<Value, String>,
}

// --- tiny local helpers ------------------------------------------------------

fn hx(n: u64) -> String {
    format!("0x{n:x}")
}

fn as_le_uint(b: &[u8]) -> Option<u64> {
    match b.len() {
        1 => Some(b[0] as u64),
        2 => Some(u16::from_le_bytes([b[0], b[1]]) as u64),
        4 => Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as u64),
        8 => Some(u64::from_le_bytes(b.try_into().ok()?)),
        _ => None,
    }
}

fn parse_perm(a: &Value) -> Permission {
    match opt_str(a, "perm") {
        None => Permission::READ_WRITE_EXECUTE,
        Some(p) => {
            let p = p.to_ascii_lowercase();
            Permission::from_flags(p.contains('r'), p.contains('w'), p.contains('x'))
        }
    }
}

fn perm_str(p: Permission) -> String {
    let mut s = String::new();
    s.push(if p.can_read() { 'r' } else { '-' });
    s.push(if p.can_write() { 'w' } else { '-' });
    s.push(if p.can_execute() { 'x' } else { '-' });
    s
}

fn parse_u128_arg(a: &Value, key: &str) -> Result<u128, String> {
    match a.get(key) {
        Some(Value::String(s)) => {
            let s = s.trim();
            if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
                u128::from_str_radix(h, 16).map_err(|e| format!("'{key}': invalid hex: {e}"))
            } else {
                s.parse::<u128>().map_err(|e| format!("'{key}': {e}"))
            }
        }
        Some(v) if v.is_u64() => Ok(v.as_u64().unwrap() as u128),
        _ => Err(format!("missing '{key}' (pass a 128-bit value as a hex string)")),
    }
}

fn arch_label(e: &libmwemu::emu::Emu) -> &'static str {
    if e.cfg.is_x64() {
        "x86_64"
    } else if e.cfg.is_aarch64() {
        "aarch64"
    } else {
        "x86"
    }
}

// --- lifecycle ---------------------------------------------------------------

fn t_open(s: &mut Server, a: &Value) -> Result<Value, String> {
    let arch = req_str(a, "arch")?;
    let mut emu = match arch.to_ascii_lowercase().as_str() {
        "x86" | "x32" | "32" | "i386" => emu32(),
        "x64" | "x86_64" | "x86-64" | "amd64" | "64" => emu64(),
        "arm64" | "aarch64" => emu_aarch64(),
        other => return Err(format!("unknown arch '{other}', use x86, x64 or arm64")),
    };
    // Quiet, offline defaults (mirrors pymwemu.init*; endpoint stays off).
    emu.cfg.console_enabled = false;
    emu.cfg.verbose = 0;
    emu.cfg.shellcode = false;
    emu.cfg.endpoint = false;
    emu.cfg.nocolors = true;

    if let Some(m) = opt_str(a, "maps_folder") {
        s.require_disk()?;
        emu.cfg.maps_folder = m.to_string();
    } else if let Some(m) = &s.default_maps {
        emu.cfg.maps_folder = m.clone();
    }

    let mode = arch_label(&emu);
    let maps = emu.cfg.maps_folder.clone();
    s.emu = Some(emu);
    Ok(json!({
        "ok": true,
        "arch": mode,
        "maps_folder": if maps.is_empty() { Value::Null } else { json!(maps) },
    }))
}

fn t_close(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let was_open = s.emu.is_some();
    s.emu = None;
    Ok(json!({ "closed": was_open }))
}

fn t_status(s: &mut Server, _a: &Value) -> Result<Value, String> {
    match &s.emu {
        None => Ok(json!({ "open": false })),
        Some(e) => Ok(json!({
            "open": true,
            "arch": arch_label(e),
            "pc": hx(e.pc()),
            "sp": hx(e.sp()),
            "pos": e.pos,
            "instruction_count": e.instruction_count,
            "maps_folder": if e.cfg.maps_folder.is_empty() { Value::Null } else { json!(e.cfg.maps_folder) },
        })),
    }
}

fn t_config(s: &mut Server, a: &Value) -> Result<Value, String> {
    // maps_folder leads to disk reads at init time -> gate it.
    if a.get("maps_folder").map(|v| !v.is_null()).unwrap_or(false) {
        s.require_disk()?;
    }
    let e = s.emu_mut()?;

    if let Some(x) = opt_u64(a, "base_address")? {
        e.cfg.code_base_addr = x;
    }
    if let Some(x) = opt_u64(a, "stack_address")? {
        e.cfg.stack_addr = x;
    }
    if let Some(x) = opt_u64(a, "entry_point")? {
        e.cfg.entry_point = x;
    }
    if let Some(x) = opt_u64(a, "verbose")? {
        e.cfg.verbose = x as u32;
    }
    if let Some(v) = a.get("max_instructions") {
        e.cfg.max_instructions = if v.is_null() { None } else { Some(parse_u64(v)?) };
    }
    if let Some(v) = a.get("timeout_secs") {
        e.cfg.timeout_secs = if v.is_null() { None } else { v.as_f64() };
    }
    if let Some(v) = a.get("max_faults") {
        e.cfg.max_faults = if v.is_null() { None } else { Some(parse_u64(v)? as u32) };
    }
    if let Some(b) = a.get("trace_mem").and_then(|v| v.as_bool()) {
        e.cfg.trace_mem = b;
    }
    if let Some(b) = a.get("trace_regs").and_then(|v| v.as_bool()) {
        e.cfg.trace_regs = b;
    }
    if let Some(b) = a.get("trace_calls").and_then(|v| v.as_bool()) {
        e.cfg.trace_calls = b;
    }
    if let Some(b) = a.get("loops").and_then(|v| v.as_bool()) {
        e.cfg.loops = b;
    }
    if let Some(b) = a.get("skip_unimplemented").and_then(|v| v.as_bool()) {
        e.cfg.skip_unimplemented = b;
        e.maps.set_banzai(b);
    }
    if let Some(m) = opt_str(a, "maps_folder") {
        e.cfg.maps_folder = m.to_string();
    }

    Ok(json!({
        "ok": true,
        "config": {
            "arch": arch_label(e),
            "base_address": hx(e.cfg.code_base_addr),
            "stack_address": hx(e.cfg.stack_addr),
            "entry_point": hx(e.cfg.entry_point),
            "verbose": e.cfg.verbose,
            "max_instructions": e.cfg.max_instructions,
            "timeout_secs": e.cfg.timeout_secs,
            "max_faults": e.cfg.max_faults,
            "trace_mem": e.cfg.trace_mem,
            "trace_regs": e.cfg.trace_regs,
            "skip_unimplemented": e.cfg.skip_unimplemented,
            "maps_folder": if e.cfg.maps_folder.is_empty() { Value::Null } else { json!(e.cfg.maps_folder) },
        }
    }))
}

// --- load / prepare ----------------------------------------------------------

fn t_load_code_bytes(s: &mut Server, a: &Value) -> Result<Value, String> {
    let bytes = get_bytes(a)?;
    let e = s.emu_mut()?;
    e.load_code_bytes(&bytes);
    Ok(json!({
        "ok": true,
        "loaded_bytes": bytes.len(),
        "base": hx(e.cfg.code_base_addr),
        "pc": hx(e.pc()),
    }))
}

fn t_load_binary(s: &mut Server, a: &Value) -> Result<Value, String> {
    s.require_disk()?;
    let path = req_str(a, "path")?.to_string();
    let e = s.emu_mut()?;
    e.load_code(&path);
    Ok(json!({ "ok": true, "path": path, "base": hx(e.base), "pc": hx(e.pc()) }))
}

fn t_load_maps(s: &mut Server, a: &Value) -> Result<Value, String> {
    s.require_disk()?;
    let folder = req_str(a, "folder")?.to_string();
    let e = s.emu_mut()?;
    e.cfg.maps_folder = folder.clone();
    Ok(json!({ "ok": true, "maps_folder": folder }))
}

fn t_init_win32(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let e = s.emu_mut()?;
    e.init_win32(false, false);
    Ok(json!({ "ok": true }))
}

fn t_init_linux64(s: &mut Server, a: &Value) -> Result<Value, String> {
    let dynamic = opt_bool(a, "dynamic", false);
    let e = s.emu_mut()?;
    e.init_linux64(dynamic);
    Ok(json!({ "ok": true, "dynamic": dynamic }))
}

fn t_alloc(s: &mut Server, a: &Value) -> Result<Value, String> {
    let name = req_str(a, "name")?.to_string();
    let size = req_u64(a, "size")?;
    let perm = parse_perm(a);
    let address = opt_u64(a, "address")?;
    let e = s.emu_mut()?;
    let base = match address {
        Some(addr) => {
            e.maps.create_map(&name, addr, size, perm)?;
            addr
        }
        None => e.alloc(&name, size, perm),
    };
    Ok(json!({
        "ok": true,
        "name": name,
        "base": hx(base),
        "size": size,
        "perm": perm_str(perm),
    }))
}

fn t_free(s: &mut Server, a: &Value) -> Result<Value, String> {
    let name = req_str(a, "name")?.to_string();
    let e = s.emu_mut()?;
    e.maps.free(&name);
    Ok(json!({ "ok": true, "freed": name }))
}

// --- memory ------------------------------------------------------------------

fn t_read_mem(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let size = req_u64(a, "size")? as usize;
    if size == 0 || size > 0x100000 {
        return Err("size must be between 1 and 1048576".to_string());
    }
    let e = s.emu()?;
    let mut bytes = Vec::with_capacity(size);
    for i in 0..size as u64 {
        match e.maps.read_byte(addr + i) {
            Some(b) => bytes.push(b),
            None => return Err(format!("unmapped address {}", hx(addr + i))),
        }
    }
    let mut out = json!({
        "address": hx(addr),
        "size": size,
        "hex": bytes_to_hex(&bytes),
    });
    if let Some(v) = as_le_uint(&bytes) {
        out["uint_le"] = json!(hx(v));
    }
    Ok(out)
}

fn t_read_string(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let wide = opt_bool(a, "wide", false);
    let e = s.emu()?;
    let st = if wide {
        e.maps.read_wide_string(addr)
    } else {
        e.maps.read_string(addr)
    };
    Ok(json!({ "address": hx(addr), "wide": wide, "string": st }))
}

fn t_write_mem(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let bytes = get_bytes(a)?;
    let e = s.emu_mut()?;
    for (i, b) in bytes.iter().enumerate() {
        if !e.maps.write_byte(addr + i as u64, *b) {
            return Err(format!(
                "unmapped address {} after writing {} of {} bytes",
                hx(addr + i as u64),
                i,
                bytes.len()
            ));
        }
    }
    Ok(json!({ "ok": true, "address": hx(addr), "written": bytes.len() }))
}

fn t_write_string(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let value = req_str(a, "string")?.to_string();
    let wide = opt_bool(a, "wide", false);
    let e = s.emu_mut()?;
    if wide {
        e.maps.write_wide_string(addr, &value);
    } else {
        e.maps.write_string(addr, &value);
    }
    Ok(json!({ "ok": true, "address": hx(addr), "wide": wide, "len": value.len() }))
}

fn t_write_int(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let value = req_u64(a, "value")?;
    let size = req_u64(a, "size")?;
    let e = s.emu_mut()?;
    let ok = match size {
        1 => e.maps.write_byte(addr, value as u8),
        2 => e.maps.write_word(addr, value as u16),
        4 => e.maps.write_dword(addr, value as u32),
        8 => e.maps.write_qword(addr, value),
        _ => return Err("size must be 1, 2, 4 or 8".to_string()),
    };
    if !ok {
        return Err(format!("unmapped address {}", hx(addr)));
    }
    Ok(json!({ "ok": true, "address": hx(addr), "value": hx(value), "size": size }))
}

fn t_memset(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let byte = req_u64(a, "byte")? as u8;
    let amount = req_u64(a, "size")? as usize;
    let e = s.emu_mut()?;
    e.maps.memset(addr, byte, amount);
    Ok(json!({ "ok": true, "address": hx(addr), "byte": byte, "size": amount }))
}

fn t_search(s: &mut Server, a: &Value) -> Result<Value, String> {
    let kind = opt_str(a, "kind").unwrap_or("string");
    let map_name = opt_str(a, "map_name");
    let e = s.emu()?;
    let found: Vec<u64> = match kind {
        "string" => {
            let kw = req_str(a, "pattern")?;
            match map_name {
                Some(m) => e.maps.search_string(kw, m).unwrap_or_default(),
                None => {
                    return Err(
                        "string search needs map_name (use kind=hex for a global search)".to_string(),
                    )
                }
            }
        }
        "hex" => {
            let bytes = hex_to_bytes(req_str(a, "pattern")?)?;
            let spaced = bytes_to_spaced_hex(&bytes);
            match map_name {
                Some(m) => e.maps.search_spaced_bytes(&spaced, m),
                None => e.maps.search_spaced_bytes_in_all(&spaced),
            }
        }
        other => return Err(format!("unknown kind '{other}', use 'string' or 'hex'")),
    };
    let addrs: Vec<String> = found.iter().map(|x| hx(*x)).collect();
    Ok(json!({ "count": addrs.len(), "matches": addrs }))
}

fn t_maps(s: &mut Server, a: &Value) -> Result<Value, String> {
    let keyword = opt_str(a, "keyword");
    let e = s.emu()?;
    let mut v = Vec::new();
    for (_base, idx) in e.maps.maps.iter() {
        if let Some(m) = e.maps.mem_slab.get(*idx) {
            let name = m.get_name();
            if let Some(k) = keyword {
                if !name.contains(k) {
                    continue;
                }
            }
            let b = m.get_base();
            let sz = m.size() as u64;
            v.push(json!({ "name": name, "base": hx(b), "size": sz, "end": hx(b + sz) }));
        }
    }
    Ok(json!({ "count": v.len(), "maps": v }))
}

// --- registers / stack -------------------------------------------------------

fn t_get_reg(s: &mut Server, a: &Value) -> Result<Value, String> {
    let reg = req_str(a, "reg")?;
    let e = s.emu()?;
    let val = if e.cfg.arch.is_aarch64() {
        e.regs_aarch64()
            .get_by_name(reg)
            .ok_or_else(|| format!("invalid aarch64 register '{reg}'"))?
    } else if e.regs().is_reg(reg) {
        e.regs().get_by_name(reg)
    } else {
        return Err(format!("invalid register '{reg}'"));
    };
    Ok(json!({ "reg": reg, "value": hx(val), "dec": val }))
}

fn t_set_reg(s: &mut Server, a: &Value) -> Result<Value, String> {
    let reg = req_str(a, "reg")?.to_string();
    let value = req_u64(a, "value")?;
    let e = s.emu_mut()?;
    let prev = if e.cfg.arch.is_aarch64() {
        let prev = e
            .regs_aarch64()
            .get_by_name(&reg)
            .ok_or_else(|| format!("invalid aarch64 register '{reg}'"))?;
        e.regs_aarch64_mut().set_by_name(&reg, value);
        prev
    } else if e.regs().is_reg(&reg) {
        let prev = e.regs().get_by_name(&reg);
        e.regs_mut().set_by_name(&reg, value);
        prev
    } else {
        return Err(format!("invalid register '{reg}'"));
    };
    Ok(json!({ "reg": reg, "value": hx(value), "previous": hx(prev) }))
}

const X64_REGS: &[&str] = &[
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15", "rip",
];
const X86_REGS: &[&str] = &[
    "eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp", "eip",
];
const ARM_REGS: &[&str] = &[
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27",
    "x28", "x29", "x30", "pc", "sp",
];

fn t_regs(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let e = s.emu()?;
    let is_arm = e.cfg.arch.is_aarch64();
    let names: &[&str] = if is_arm {
        ARM_REGS
    } else if e.cfg.is_x64() {
        X64_REGS
    } else {
        X86_REGS
    };
    let mut m = serde_json::Map::new();
    for n in names {
        let val = if is_arm {
            e.regs_aarch64().get_by_name(n).unwrap_or(0)
        } else if e.regs().is_reg(n) {
            e.regs().get_by_name(n)
        } else {
            0
        };
        m.insert((*n).to_string(), json!(hx(val)));
    }
    Ok(json!({ "arch": arch_label(e), "regs": Value::Object(m) }))
}

fn t_get_xmm(s: &mut Server, a: &Value) -> Result<Value, String> {
    let reg = req_str(a, "reg")?;
    let e = s.emu()?;
    if e.cfg.arch.is_aarch64() {
        return Err("xmm registers are not available on aarch64".to_string());
    }
    if e.regs().is_xmm_by_name(reg) {
        let v = e.regs().get_xmm_by_name(reg);
        Ok(json!({ "reg": reg, "value": format!("0x{v:x}") }))
    } else {
        Err(format!("invalid xmm register '{reg}'"))
    }
}

fn t_set_xmm(s: &mut Server, a: &Value) -> Result<Value, String> {
    let reg = req_str(a, "reg")?.to_string();
    let value = parse_u128_arg(a, "value")?;
    let e = s.emu_mut()?;
    if e.cfg.arch.is_aarch64() {
        return Err("xmm registers are not available on aarch64".to_string());
    }
    if e.regs().is_xmm_by_name(&reg) {
        let prev = e.regs().get_xmm_by_name(&reg);
        e.regs_mut().set_xmm_by_name(&reg, value);
        Ok(json!({ "reg": reg, "value": format!("0x{value:x}"), "previous": format!("0x{prev:x}") }))
    } else {
        Err(format!("invalid xmm register '{reg}'"))
    }
}

fn t_stack_push(s: &mut Server, a: &Value) -> Result<Value, String> {
    let value = req_u64(a, "value")?;
    let e = s.emu_mut()?;
    let size = opt_u64(a, "size")?.unwrap_or(if e.cfg.is_x64() { 8 } else { 4 });
    let ok = if size == 8 {
        e.stack_push64(value)
    } else {
        e.stack_push32(value as u32)
    };
    if !ok {
        return Err("stack push failed".to_string());
    }
    Ok(json!({ "ok": true, "value": hx(value), "sp": hx(e.sp()) }))
}

fn t_stack_pop(s: &mut Server, a: &Value) -> Result<Value, String> {
    let e = s.emu_mut()?;
    let size = opt_u64(a, "size")?.unwrap_or(if e.cfg.is_x64() { 8 } else { 4 });
    let val = if size == 8 {
        e.stack_pop64(false)
    } else {
        e.stack_pop32(false).map(|v| v as u64)
    };
    match val {
        Some(v) => Ok(json!({ "value": hx(v), "sp": hx(e.sp()) })),
        None => Err("stack pop failed".to_string()),
    }
}

// --- execution ---------------------------------------------------------------

fn t_step(s: &mut Server, a: &Value) -> Result<Value, String> {
    let count = opt_u64(a, "count")?.unwrap_or(1);
    let e = s.emu_mut()?;
    let mut executed = 0u64;
    let mut keep_going = true;
    for _ in 0..count {
        if !e.step() {
            keep_going = false;
            break;
        }
        executed += 1;
    }
    Ok(json!({
        "steps_executed": executed,
        "continued": keep_going,
        "pc": hx(e.pc()),
        "pos": e.pos,
    }))
}

fn t_run(s: &mut Server, a: &Value) -> Result<Value, String> {
    let end = opt_u64(a, "end_address")?;
    let e = s.emu_mut()?;
    if let Some(mi) = opt_u64(a, "max_instructions")? {
        e.cfg.max_instructions = Some(mi);
    }
    // The server is synchronous; an unbounded run with no terminating address
    // would hang it. Require a bound.
    if end.is_none() && e.cfg.max_instructions.is_none() && e.cfg.timeout_secs.is_none() {
        return Err("refusing an unbounded run that could hang the server: pass \
                    end_address, or set max_instructions / timeout_secs"
            .to_string());
    }
    match e.run(end) {
        Ok(pc) => Ok(json!({
            "pc": hx(pc),
            "pos": e.pos,
            "instruction_count": e.instruction_count,
        })),
        Err(err) => Err(err.message),
    }
}

fn t_run_to(s: &mut Server, a: &Value) -> Result<Value, String> {
    let position = req_u64(a, "position")?;
    let e = s.emu_mut()?;
    match e.run_to(position) {
        Ok(pc) => Ok(json!({ "pc": hx(pc), "pos": e.pos })),
        Err(err) => Err(err.message),
    }
}

fn t_run_until_return(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let e = s.emu_mut()?;
    match e.run_until_ret() {
        Ok(pc) => Ok(json!({ "pc": hx(pc), "pos": e.pos })),
        Err(err) => Err(err.message),
    }
}

fn t_run_until_apicall(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let e = s.emu_mut()?;
    e.skip_apicall = true;
    e.is_break_on_api = true;
    let _ = e.run(None);
    match e.its_apicall {
        Some(addr) => {
            e.skip_apicall = false;
            let name = e.api_addr_to_name(addr);
            let new_pc = e.pc() + e.last_instruction_size as u64;
            e.set_pc(new_pc);
            Ok(json!({ "address": hx(addr), "name": name, "pc": hx(new_pc) }))
        }
        None => Err("no API call was reached".to_string()),
    }
}

fn t_call(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let args_v = a
        .get("args")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut args = Vec::with_capacity(args_v.len());
    for (i, v) in args_v.iter().enumerate() {
        args.push(parse_u64(v).map_err(|er| format!("args[{i}]: {er}"))?);
    }
    let abi = opt_str(a, "abi");
    let e = s.emu_mut()?;

    let ret: u64 = if matches!(abi, Some("linux64") | Some("linux")) {
        e.linux_call64(addr, &args).map_err(|er| er.message)?
    } else if matches!(abi, Some("x86") | Some("stdcall32") | Some("cdecl32") | Some("32"))
        || (abi.is_none() && !e.cfg.is_x64())
    {
        let a32: Vec<u32> = args.iter().map(|x| *x as u32).collect();
        e.call32(addr, &a32).map_err(|er| er.message)? as u64
    } else {
        e.call64(addr, &args).map_err(|er| er.message)?
    };
    Ok(json!({ "return": hx(ret), "pc": hx(e.pc()) }))
}

fn t_set_pc(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let e = s.emu_mut()?;
    e.set_pc(addr);
    Ok(json!({ "ok": true, "pc": hx(addr) }))
}

// --- inspect / aux -----------------------------------------------------------

fn t_disassemble(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let count = opt_u64(a, "count")?.unwrap_or(10) as u32;
    let e = s.emu_mut()?;
    let text = e.disassemble(addr, count);
    Ok(json!(text))
}

fn t_call_stack(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let e = s.emu()?;
    let cs = e.call_stack().clone();
    let v: Vec<Value> = cs
        .iter()
        .map(|(from, to)| json!({ "from": hx(*from), "to": hx(*to) }))
        .collect();
    Ok(json!({ "depth": v.len(), "call_stack": v }))
}

fn t_prev_mnemonic(s: &mut Server, _a: &Value) -> Result<Value, String> {
    let e = s.emu_mut()?;
    if let Some(d) = e.last_decoded {
        let st = e.format_instruction(&d);
        Ok(json!({ "instruction": st }))
    } else {
        Err("no instruction decoded yet".to_string())
    }
}

fn t_api_addr_to_name(s: &mut Server, a: &Value) -> Result<Value, String> {
    let addr = req_u64(a, "address")?;
    let e = s.emu_mut()?;
    let name = e.api_addr_to_name(addr);
    Ok(json!({ "address": hx(addr), "name": name }))
}

fn t_api_name_to_addr(s: &mut Server, a: &Value) -> Result<Value, String> {
    let name = req_str(a, "name")?.to_string();
    let e = s.emu_mut()?;
    let addr = e.api_name_to_addr(&name);
    Ok(json!({ "name": name, "address": hx(addr) }))
}

fn t_bp(s: &mut Server, a: &Value) -> Result<Value, String> {
    let action = req_str(a, "action")?;
    let e = s.emu_mut()?;
    match action {
        "list" => Ok(json!({
            "addr": e.bp.addr.iter().map(|x| hx(*x)).collect::<Vec<_>>(),
            "inst": e.bp.instruction.clone(),
            "mem_read": e.bp.mem_read_addr.iter().map(|x| hx(*x)).collect::<Vec<_>>(),
            "mem_write": e.bp.mem_write_addr.iter().map(|x| hx(*x)).collect::<Vec<_>>(),
        })),
        "clear" => {
            e.bp.clear_bp();
            Ok(json!({ "ok": true, "cleared": true }))
        }
        "set" => {
            let kind = opt_str(a, "kind").unwrap_or("addr");
            let addr = req_u64(a, "address")?;
            match kind {
                "addr" => e.bp.add_bp(addr),
                "inst" => e.bp.add_bp_instruction(addr),
                "mem_read" => e.bp.add_bp_mem_read(addr),
                "mem_write" => e.bp.add_bp_mem_write(addr),
                other => return Err(format!("unknown bp kind '{other}'")),
            }
            Ok(json!({ "ok": true, "kind": kind, "address": hx(addr) }))
        }
        other => Err(format!("unknown action '{other}', use set | clear | list")),
    }
}

// --- schemas -----------------------------------------------------------------

fn obj(props: Value, required: &[&str]) -> Value {
    json!({
        "type": "object",
        "properties": props,
        "required": required,
    })
}

fn sc_open() -> Value {
    obj(
        json!({
            "arch": { "type": "string", "description": "x86 (32-bit), x64 (x86_64) or arm64 (aarch64)" },
            "maps_folder": { "type": "string", "description": "optional trusted maps folder (disk-gated)" }
        }),
        &["arch"],
    )
}
fn sc_empty() -> Value {
    obj(json!({}), &[])
}
fn sc_config() -> Value {
    obj(
        json!({
            "base_address": { "type": ["string","integer"] },
            "stack_address": { "type": ["string","integer"] },
            "entry_point": { "type": ["string","integer"] },
            "verbose": { "type": "integer", "description": "0..3; output goes to stderr" },
            "max_instructions": { "type": ["string","integer","null"] },
            "timeout_secs": { "type": ["number","null"] },
            "max_faults": { "type": ["integer","null"] },
            "trace_mem": { "type": "boolean" },
            "trace_regs": { "type": "boolean" },
            "trace_calls": { "type": "boolean" },
            "loops": { "type": "boolean" },
            "skip_unimplemented": { "type": "boolean", "description": "banzai: keep going on unimplemented ops/apis" },
            "maps_folder": { "type": "string", "description": "disk-gated" }
        }),
        &[],
    )
}
fn sc_load_code_bytes() -> Value {
    obj(
        json!({
            "hex": { "type": "string", "description": "machine code as hex (spaced or contiguous)" },
            "bytes": { "type": "array", "items": { "type": "integer" } }
        }),
        &[],
    )
}
fn sc_load_binary() -> Value {
    obj(json!({ "path": { "type": "string" } }), &["path"])
}
fn sc_load_maps() -> Value {
    obj(json!({ "folder": { "type": "string" } }), &["folder"])
}
fn sc_init_linux64() -> Value {
    obj(json!({ "dynamic": { "type": "boolean" } }), &[])
}
fn sc_alloc() -> Value {
    obj(
        json!({
            "name": { "type": "string" },
            "size": { "type": ["string","integer"] },
            "address": { "type": ["string","integer"], "description": "optional fixed base (alloc_at)" },
            "perm": { "type": "string", "description": "permission like 'rwx', 'r-x' (default rwx)" }
        }),
        &["name", "size"],
    )
}
fn sc_free() -> Value {
    obj(json!({ "name": { "type": "string" } }), &["name"])
}
fn sc_read_mem() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "size": { "type": ["string","integer"], "description": "number of bytes (1..1048576)" }
        }),
        &["address", "size"],
    )
}
fn sc_read_string() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "wide": { "type": "boolean", "description": "UTF-16 wide string" }
        }),
        &["address"],
    )
}
fn sc_write_mem() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "hex": { "type": "string" },
            "bytes": { "type": "array", "items": { "type": "integer" } }
        }),
        &["address"],
    )
}
fn sc_write_string() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "string": { "type": "string" },
            "wide": { "type": "boolean" }
        }),
        &["address", "string"],
    )
}
fn sc_write_int() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "value": { "type": ["string","integer"] },
            "size": { "type": "integer", "description": "1, 2, 4 or 8" }
        }),
        &["address", "value", "size"],
    )
}
fn sc_memset() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "byte": { "type": "integer" },
            "size": { "type": ["string","integer"] }
        }),
        &["address", "byte", "size"],
    )
}
fn sc_search() -> Value {
    obj(
        json!({
            "pattern": { "type": "string", "description": "substring (kind=string) or hex bytes (kind=hex)" },
            "kind": { "type": "string", "enum": ["string","hex"] },
            "map_name": { "type": "string", "description": "limit to a map; omit for global (hex only)" }
        }),
        &["pattern"],
    )
}
fn sc_maps() -> Value {
    obj(json!({ "keyword": { "type": "string" } }), &[])
}
fn sc_get_reg() -> Value {
    obj(json!({ "reg": { "type": "string" } }), &["reg"])
}
fn sc_set_reg() -> Value {
    obj(
        json!({ "reg": { "type": "string" }, "value": { "type": ["string","integer"] } }),
        &["reg", "value"],
    )
}
fn sc_get_xmm() -> Value {
    obj(json!({ "reg": { "type": "string" } }), &["reg"])
}
fn sc_set_xmm() -> Value {
    obj(
        json!({ "reg": { "type": "string" }, "value": { "type": "string", "description": "128-bit hex" } }),
        &["reg", "value"],
    )
}
fn sc_stack_push() -> Value {
    obj(
        json!({ "value": { "type": ["string","integer"] }, "size": { "type": "integer", "description": "4 or 8 bytes" } }),
        &["value"],
    )
}
fn sc_stack_pop() -> Value {
    obj(json!({ "size": { "type": "integer" } }), &[])
}
fn sc_step() -> Value {
    obj(json!({ "count": { "type": "integer", "description": "instructions to step (default 1)" } }), &[])
}
fn sc_run() -> Value {
    obj(
        json!({
            "end_address": { "type": ["string","integer"], "description": "stop when PC reaches it" },
            "max_instructions": { "type": ["string","integer"] }
        }),
        &[],
    )
}
fn sc_run_to() -> Value {
    obj(json!({ "position": { "type": ["string","integer"], "description": "instruction count to reach" } }), &["position"])
}
fn sc_call() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "args": { "type": "array", "items": { "type": ["string","integer"] } },
            "abi": { "type": "string", "enum": ["win64","linux64","stdcall32"], "description": "default: by arch" }
        }),
        &["address"],
    )
}
fn sc_set_pc() -> Value {
    obj(json!({ "address": { "type": ["string","integer"] } }), &["address"])
}
fn sc_disassemble() -> Value {
    obj(
        json!({
            "address": { "type": ["string","integer"] },
            "count": { "type": "integer", "description": "number of instructions (default 10)" }
        }),
        &["address"],
    )
}
fn sc_api_addr() -> Value {
    obj(json!({ "address": { "type": ["string","integer"] } }), &["address"])
}
fn sc_api_name() -> Value {
    obj(json!({ "name": { "type": "string" } }), &["name"])
}
fn sc_bp() -> Value {
    obj(
        json!({
            "action": { "type": "string", "enum": ["set","clear","list"] },
            "kind": { "type": "string", "enum": ["addr","inst","mem_read","mem_write"] },
            "address": { "type": ["string","integer"] }
        }),
        &["action"],
    )
}

// --- registry ----------------------------------------------------------------

pub const TOOLS: &[ToolDef] = &[
    ToolDef { name: "mwemu_open", description: "Open a new emulator session for an architecture (x86, x64 or arm64). Replaces any previous session.", schema: sc_open, handler: t_open },
    ToolDef { name: "mwemu_close", description: "Close the current emulator session and free it.", schema: sc_empty, handler: t_close },
    ToolDef { name: "mwemu_status", description: "Report whether a session is open and its arch, PC, SP and instruction position.", schema: sc_empty, handler: t_status },
    ToolDef { name: "mwemu_config", description: "Configure the open session: base/stack/entry addresses, verbosity, execution limits (max_instructions, timeout_secs, max_faults), tracing and banzai. All fields optional.", schema: sc_config, handler: t_config },

    ToolDef { name: "mwemu_load_code_bytes", description: "Load raw machine code/shellcode into the session from inline bytes (hex string or byte array). The safe way to feed code.", schema: sc_load_code_bytes, handler: t_load_code_bytes },
    ToolDef { name: "mwemu_load_binary", description: "Load a PE/ELF/Mach-O binary from a filesystem path (disk-gated; disabled in sandbox mode).", schema: sc_load_binary, handler: t_load_binary },
    ToolDef { name: "mwemu_load_maps", description: "Set the 32/64-bit maps folder for a realistic memory layout (disk-gated).", schema: sc_load_maps, handler: t_load_maps },
    ToolDef { name: "mwemu_init_win32", description: "Initialise the Windows simulation environment (PEB/TEB/LDR/DLLs) for the open session.", schema: sc_empty, handler: t_init_win32 },
    ToolDef { name: "mwemu_init_linux64", description: "Initialise the Linux simulation environment for the open session.", schema: sc_init_linux64, handler: t_init_linux64 },
    ToolDef { name: "mwemu_alloc", description: "Allocate a named memory region (optionally at a fixed address) and return its base. Permission defaults to rwx.", schema: sc_alloc, handler: t_alloc },
    ToolDef { name: "mwemu_free", description: "Free a named memory region.", schema: sc_free, handler: t_free },

    ToolDef { name: "mwemu_read_mem", description: "Read N bytes of emulated memory; returns hex plus a little-endian integer for sizes 1/2/4/8.", schema: sc_read_mem, handler: t_read_mem },
    ToolDef { name: "mwemu_read_string", description: "Read a null-terminated ASCII (or wide UTF-16) string from memory.", schema: sc_read_string, handler: t_read_string },
    ToolDef { name: "mwemu_write_mem", description: "Write raw bytes (hex string or byte array) to emulated memory.", schema: sc_write_mem, handler: t_write_mem },
    ToolDef { name: "mwemu_write_string", description: "Write an ASCII (or wide UTF-16) string to memory.", schema: sc_write_string, handler: t_write_string },
    ToolDef { name: "mwemu_write_int", description: "Write a little-endian integer of 1, 2, 4 or 8 bytes to memory.", schema: sc_write_int, handler: t_write_int },
    ToolDef { name: "mwemu_memset", description: "Fill a memory region with a repeated byte.", schema: sc_memset, handler: t_memset },
    ToolDef { name: "mwemu_search", description: "Search memory for a substring (kind=string, needs map_name) or hex bytes (kind=hex, optional map_name for global).", schema: sc_search, handler: t_search },
    ToolDef { name: "mwemu_maps", description: "List the memory maps (name, base, size, end), optionally filtered by keyword.", schema: sc_maps, handler: t_maps },

    ToolDef { name: "mwemu_get_reg", description: "Read a register by name (e.g. rax, eax, x0).", schema: sc_get_reg, handler: t_get_reg },
    ToolDef { name: "mwemu_set_reg", description: "Set a register by name; returns the previous value.", schema: sc_set_reg, handler: t_set_reg },
    ToolDef { name: "mwemu_regs", description: "Dump the general-purpose registers plus the program counter.", schema: sc_empty, handler: t_regs },
    ToolDef { name: "mwemu_get_xmm", description: "Read a 128-bit XMM register by name (x86 only).", schema: sc_get_xmm, handler: t_get_xmm },
    ToolDef { name: "mwemu_set_xmm", description: "Set a 128-bit XMM register (x86 only); value as hex string.", schema: sc_set_xmm, handler: t_set_xmm },
    ToolDef { name: "mwemu_stack_push", description: "Push a value onto the stack (4 or 8 bytes; default by arch).", schema: sc_stack_push, handler: t_stack_push },
    ToolDef { name: "mwemu_stack_pop", description: "Pop a value from the stack (4 or 8 bytes; default by arch).", schema: sc_stack_pop, handler: t_stack_pop },

    ToolDef { name: "mwemu_step", description: "Single-step the emulator N instructions (default 1).", schema: sc_step, handler: t_step },
    ToolDef { name: "mwemu_run", description: "Run until end_address (or until a configured limit). Requires a bound to avoid hanging the server.", schema: sc_run, handler: t_run },
    ToolDef { name: "mwemu_run_to", description: "Run until a given instruction position (count).", schema: sc_run_to, handler: t_run_to },
    ToolDef { name: "mwemu_run_until_return", description: "Run until the next RET (step over).", schema: sc_empty, handler: t_run_until_return },
    ToolDef { name: "mwemu_run_until_apicall", description: "Run until the next Windows API call; returns its address and name.", schema: sc_empty, handler: t_run_until_apicall },
    ToolDef { name: "mwemu_call", description: "Call a function at an address with integer args (pushed in reverse); abi defaults to the session arch.", schema: sc_call, handler: t_call },
    ToolDef { name: "mwemu_set_pc", description: "Set the program counter (RIP/EIP/PC).", schema: sc_set_pc, handler: t_set_pc },

    ToolDef { name: "mwemu_disassemble", description: "Disassemble N instructions at an address (returns text).", schema: sc_disassemble, handler: t_disassemble },
    ToolDef { name: "mwemu_call_stack", description: "Return the current call stack (from/to pairs).", schema: sc_empty, handler: t_call_stack },
    ToolDef { name: "mwemu_prev_mnemonic", description: "Return the last decoded instruction with operands.", schema: sc_empty, handler: t_prev_mnemonic },
    ToolDef { name: "mwemu_api_addr_to_name", description: "Resolve an API address to its name.", schema: sc_api_addr, handler: t_api_addr_to_name },
    ToolDef { name: "mwemu_api_name_to_addr", description: "Resolve an API name to its address.", schema: sc_api_name, handler: t_api_name_to_addr },

    ToolDef { name: "mwemu_bp", description: "Manage breakpoints: action set|clear|list, kind addr|inst|mem_read|mem_write.", schema: sc_bp, handler: t_bp },
];
