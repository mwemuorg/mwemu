//! pthread lifecycle on top of the emulator thread scheduler: create, join,
//! exit, self, detach. Threads are interleaved by the multi-threaded run loop.
use super::libc_extra::trace;
use crate::api::abi::ApiAbi;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::threading::context::ThreadContext;

const THREAD_STACK_SIZE: u64 = 0x80000; // macOS default for secondary threads
const EAGAIN: u64 = 35;
const ESRCH: u64 = 3;
const EDEADLK: u64 = 11;

/// Dispatch `symbol` if it is implemented here. Returns false otherwise.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let name = symbol.strip_prefix('_').unwrap_or(symbol);
    match name {
        "pthread_create" => api_pthread_create(emu),
        "pthread_join" => api_pthread_join(emu),
        "pthread_exit" => api_pthread_exit(emu),
        "pthread_self" => api_pthread_self(emu),
        "pthread_detach" => api_pthread_detach(emu),
        "pthread_equal" => api_pthread_equal(emu),
        _ => return false,
    }
    true
}

fn api_pthread_create(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let thread_ptr = abi.arg(emu, 0);
    let start = abi.arg(emu, 2);
    let param = abi.arg(emu, 3);

    let Some(thread_return) = super::sentinel_addr(emu, super::THREAD_RETURN) else {
        trace(emu, "pthread_create() -> EAGAIN (aarch64 only)");
        abi.set_ret(emu, EAGAIN);
        return;
    };

    let id = 0x1000 + emu.threads.len() as u64;
    let stack = emu.maps.map(
        &format!("pthread_stack_{:x}", id),
        THREAD_STACK_SIZE,
        Permission::READ_WRITE,
    );

    let mut thread = ThreadContext::new(id, emu.cfg.arch);
    {
        let regs = thread.regs_aarch64_mut();
        *regs = *emu.regs_aarch64();
        regs.pc = start;
        regs.x[0] = param;
        regs.x[29] = 0;
        regs.x[30] = thread_return;
        regs.sp = (stack + THREAD_STACK_SIZE) & !0xf;
    }
    emu.threads.push(thread);
    emu.cfg.enable_threading = true;

    if thread_ptr != 0 {
        emu.maps.write_qword(thread_ptr, id);
    }
    trace(
        emu,
        &format!(
            "pthread_create(start=0x{:x}, arg=0x{:x}) -> 0 tid=0x{:x}",
            start, param, id
        ),
    );
    abi.set_ret(emu, 0);
}

fn api_pthread_join(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let target = abi.arg(emu, 0);
    let retval_ptr = abi.arg(emu, 1);

    let Some(idx) = emu.threads.iter().position(|t| t.id == target) else {
        trace(emu, &format!("pthread_join(0x{:x}) -> ESRCH", target));
        abi.set_ret(emu, ESRCH);
        return;
    };
    if idx == emu.current_thread_id {
        trace(emu, &format!("pthread_join(0x{:x}) -> EDEADLK", target));
        abi.set_ret(emu, EDEADLK);
        return;
    }

    if let Some(value) = emu.threads[idx].exit_value {
        if retval_ptr != 0 {
            emu.maps.write_qword(retval_ptr, value);
        }
        trace(
            emu,
            &format!("pthread_join(0x{:x}) -> 0 retval=0x{:x}", target, value),
        );
        abi.set_ret(emu, 0);
        return;
    }

    // Block until the target exits; thread_exit() completes the call.
    trace(emu, &format!("pthread_join(0x{:x}) -> blocking", target));
    let cur = emu.current_thread_id;
    emu.threads[cur].joining = Some((target, retval_ptr));
}

fn api_pthread_exit(emu: &mut Emu) {
    let value = ApiAbi::from_emu(emu).arg(emu, 0);
    thread_exit(emu, value);
}

fn api_pthread_self(emu: &mut Emu) {
    let id = emu.threads[emu.current_thread_id].id;
    trace(emu, &format!("pthread_self() -> 0x{:x}", id));
    ApiAbi::from_emu(emu).set_ret(emu, id);
}

fn api_pthread_detach(emu: &mut Emu) {
    let target = ApiAbi::from_emu(emu).arg(emu, 0);
    trace(emu, &format!("pthread_detach(0x{:x}) -> 0", target));
    ApiAbi::from_emu(emu).set_ret(emu, 0);
}

fn api_pthread_equal(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let equal = (abi.arg(emu, 0) == abi.arg(emu, 1)) as u64;
    trace(emu, &format!("pthread_equal() -> {}", equal));
    abi.set_ret(emu, equal);
}

/// Finish the current thread with `value`, completing any pending joins on
/// it. Stops the emulator once no thread is left to run.
fn thread_exit(emu: &mut Emu, value: u64) {
    let cur = emu.current_thread_id;
    let tid = emu.threads[cur].id;
    emu.threads[cur].exit_value = Some(value);
    trace(
        emu,
        &format!("pthread_exit(0x{:x}) tid=0x{:x} finished", value, tid),
    );

    for i in 0..emu.threads.len() {
        let Some((target, retval_ptr)) = emu.threads[i].joining else {
            continue;
        };
        if target != tid {
            continue;
        }
        if retval_ptr != 0 {
            emu.maps.write_qword(retval_ptr, value);
        }
        emu.threads[i].joining = None;
        emu.threads[i].regs_aarch64_mut().x[0] = 0;
    }

    if emu.threads.iter().all(|t| t.exit_value.is_some()) {
        emu.stop();
    }
}
