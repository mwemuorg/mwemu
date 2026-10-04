//! Process APIs: fork family, wait family and pid identity. The process model
//! itself (snapshot/restore, zombies) lives in `threading::process`.
use super::libc_extra::trace;
use crate::api::abi::ApiAbi;
use crate::emu::Emu;

const RUSAGE_SIZE: u64 = 144;

/// Dispatch `symbol` if it is implemented here. Returns false otherwise.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let name = symbol.strip_prefix('_').unwrap_or(symbol);
    match name {
        "fork" | "vfork" => api_fork(emu, name),
        "waitpid" => api_waitpid(emu),
        "wait" => api_wait(emu),
        "wait4" => api_wait4(emu),
        "getpid" => api_getpid(emu),
        "getppid" => api_getppid(emu),
        _ => return false,
    }
    true
}

fn api_fork(emu: &mut Emu, name: &str) {
    let parent = emu.processes.pid;
    let child = emu.fork_begin();
    trace(
        emu,
        &format!(
            "{}() pid {} -> running child pid {} to completion",
            name, parent, child
        ),
    );
    ApiAbi::from_emu(emu).set_ret(emu, 0);
}

/// Shared by the wait family: reap a child, store its status, return its pid
/// (or -1 when there is no matching child).
fn reap_into(emu: &mut Emu, name: &str, pid: i64, status_ptr: u64) -> u64 {
    match emu.reap_child(pid) {
        Some((child, status)) => {
            if status_ptr != 0 {
                emu.maps.write_dword(status_ptr, status as u32);
            }
            trace(
                emu,
                &format!("{}({}) -> {} status=0x{:x}", name, pid, child, status),
            );
            child
        }
        None => {
            trace(emu, &format!("{}({}) -> -1 (no child)", name, pid));
            -1i64 as u64
        }
    }
}

fn api_waitpid(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let pid = abi.arg(emu, 0) as i32 as i64;
    let status_ptr = abi.arg(emu, 1);
    let ret = reap_into(emu, "waitpid", pid, status_ptr);
    abi.set_ret(emu, ret);
}

fn api_wait(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let status_ptr = abi.arg(emu, 0);
    let ret = reap_into(emu, "wait", -1, status_ptr);
    abi.set_ret(emu, ret);
}

fn api_wait4(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let pid = abi.arg(emu, 0) as i32 as i64;
    let status_ptr = abi.arg(emu, 1);
    let rusage = abi.arg(emu, 3);
    let ret = reap_into(emu, "wait4", pid, status_ptr);
    if rusage != 0 && ret != -1i64 as u64 {
        emu.maps.memset(rusage, 0, RUSAGE_SIZE as usize);
    }
    abi.set_ret(emu, ret);
}

fn api_getpid(emu: &mut Emu) {
    let pid = emu.processes.pid;
    trace(emu, &format!("getpid() -> {}", pid));
    ApiAbi::from_emu(emu).set_ret(emu, pid);
}

fn api_getppid(emu: &mut Emu) {
    let ppid = emu.processes.ppid;
    trace(emu, &format!("getppid() -> {}", ppid));
    ApiAbi::from_emu(emu).set_ret(emu, ppid);
}
