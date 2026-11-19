//! Process APIs: fork family, wait family and pid identity. The process model
//! itself (snapshot/restore, zombies) lives in `threading::process`.
use super::libc_extra::trace;
use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::threading::process::AtFork;

const RUSAGE_SIZE: u64 = 144;

/// Dispatch `symbol` if it is implemented here. Returns false otherwise.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let name = symbol.strip_prefix('_').unwrap_or(symbol);
    dispatch_api!(name, {
        "fork" | "vfork" => api_fork(emu, name),
        "waitpid" => api_waitpid(emu),
        "wait" => api_wait(emu),
        "wait4" => api_wait4(emu),
        "getpid" => api_getpid(emu),
        "getppid" => api_getppid(emu),
        "pthread_atfork" => api_pthread_atfork(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn api_fork(emu: &mut Emu, name: &str) {
    let call = UnixApi;
    run_atfork(emu, AtForkPhase::Prepare);
    let parent = emu.processes.pid;
    let child = emu.fork_begin();
    run_atfork(emu, AtForkPhase::Child);
    trace(
        emu,
        &format!(
            "{}() pid {} -> running child pid {} to completion",
            name, parent, child
        ),
    );
    call.set_return(emu, 0);
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
    let call = UnixApi;
    let pid = api_try!(emu, call.arg_i32(emu, 0)) as i64;
    let status_ptr = api_try!(emu, call.arg(emu, 1));
    let ret = reap_into(emu, "waitpid", pid, status_ptr);
    call.set_return(emu, ret);
}

fn api_wait(emu: &mut Emu) {
    let call = UnixApi;
    let status_ptr = api_try!(emu, call.arg(emu, 0));
    let ret = reap_into(emu, "wait", -1, status_ptr);
    call.set_return(emu, ret);
}

fn api_wait4(emu: &mut Emu) {
    let call = UnixApi;
    let pid = api_try!(emu, call.arg_i32(emu, 0)) as i64;
    let status_ptr = api_try!(emu, call.arg(emu, 1));
    let rusage = api_try!(emu, call.arg(emu, 3));
    let ret = reap_into(emu, "wait4", pid, status_ptr);
    if rusage != 0 && ret != -1i64 as u64 {
        emu.maps.memset(rusage, 0, RUSAGE_SIZE as usize);
    }
    call.set_return(emu, ret);
}

fn api_getpid(emu: &mut Emu) {
    let call = UnixApi;
    let pid = emu.processes.pid;
    trace(emu, &format!("getpid() -> {}", pid));
    call.set_return(emu, pid);
}

fn api_getppid(emu: &mut Emu) {
    let call = UnixApi;
    let ppid = emu.processes.ppid;
    trace(emu, &format!("getppid() -> {}", ppid));
    call.set_return(emu, ppid);
}

fn api_pthread_atfork(emu: &mut Emu) {
    let call = UnixApi;
    let handlers = AtFork {
        prepare: api_try!(emu, call.arg(emu, 0)),
        parent: api_try!(emu, call.arg(emu, 1)),
        child: api_try!(emu, call.arg(emu, 2)),
    };
    emu.processes.atfork.push(handlers);
    trace(
        emu,
        &format!(
            "pthread_atfork(prepare=0x{:x}, parent=0x{:x}, child=0x{:x}) -> 0",
            handlers.prepare, handlers.parent, handlers.child
        ),
    );
    call.set_return(emu, 0);
}

pub(super) enum AtForkPhase {
    Prepare,
    Parent,
    Child,
}

/// Run the registered atfork handlers for `phase` on the calling thread:
/// prepare in reverse registration order, parent and child in order.
pub(super) fn run_atfork(emu: &mut Emu, phase: AtForkPhase) {
    let mut handlers: Vec<u64> = emu
        .processes
        .atfork
        .iter()
        .map(|h| match phase {
            AtForkPhase::Prepare => h.prepare,
            AtForkPhase::Parent => h.parent,
            AtForkPhase::Child => h.child,
        })
        .filter(|addr| *addr != 0)
        .collect();
    if matches!(phase, AtForkPhase::Prepare) {
        handlers.reverse();
    }
    if !handlers.is_empty() && !emu.cfg.arch.is_aarch64() {
        log::warn!("pthread_atfork handlers are only run on aarch64");
        return;
    }
    for addr in handlers {
        trace(emu, &format!("atfork handler 0x{:x}", addr));
        if let Err(e) = emu.aarch64_call64(addr, &[]) {
            log::warn!("atfork handler 0x{:x} failed: {}", addr, e);
        }
    }
}
