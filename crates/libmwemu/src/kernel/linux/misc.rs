//! Time, randomness, capabilities and deferred execution.
//!
//! The deferred-work handlers are the interesting ones. Work items, timers and
//! RCU callbacks are how real drivers postpone a free — "unregister now, free
//! later" is the shape of most kernel use-after-free bugs. Running the callback
//! immediately would hide the window; queueing it and letting the caller decide
//! when to drain (see [`Emu::kernel_run_deferred`]) keeps the window visible.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::DeferredCall;
use crate::kernel::{HandlerRegistry, ignore_call, kernel_handlers, return_one, return_zero};

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- deferred work ----------------------------------------------------
        // struct work_struct { atomic_long_t data; struct list_head entry;
        //                      work_func_t func; }  -> func at offset 0x18
        "queue_work_on" | "schedule_work" | "queue_work" => queue_work_on,
        "queue_delayed_work_on" | "schedule_delayed_work" => queue_delayed_work_on,
        "flush_work"
        | "cancel_work_sync"
        | "cancel_delayed_work"
        | "cancel_delayed_work_sync"
        | "flush_scheduled_work"
        | "flush_workqueue"
        | "destroy_workqueue"
        | "__flush_workqueue" => drain_work_stub,
        "alloc_workqueue" | "__alloc_workqueue" | "__alloc_workqueue_key" => workqueue_stub,
        "__init_work"
        | "__INIT_WORK"
        | "init_timer_key"
        | "timer_setup"
        | "__init_timer"
        | "__init_timer_on_stack" => return_zero,

        // struct timer_list { ...; void (*function)(struct timer_list *); }
        // function sits at offset 0x18 on x86_64.
        "mod_timer" | "add_timer" | "timer_reduce" => mod_timer,
        "del_timer"
        | "del_timer_sync"
        | "timer_delete"
        | "timer_delete_sync"
        | "timer_shutdown_sync" => cancel_timer_stub,
        // call_rcu(head, func): func receives the rcu_head pointer.
        "call_rcu" | "call_rcu_hurry" => call_rcu,
        "kvfree_call_rcu" | "kfree_call_rcu" => kvfree_call_rcu,

        // --- kthreads -----------------------------------------------------------
        "kthread_create_on_node" | "kthread_create" | "kthread_run" => task_stub,
        "wake_up_process" | "kthread_stop" | "kthread_should_stop" => return_zero,

        // --- time and delays ------------------------------------------------------
        "msleep"
        | "msleep_interruptible"
        | "ssleep"
        | "usleep_range"
        | "usleep_range_state"
        | "__const_udelay"
        | "__udelay"
        | "__ndelay"
        | "fsleep" => delay_stub,
        "ktime_get"
        | "ktime_get_real"
        | "ktime_get_boottime"
        | "ktime_get_ns"
        | "ktime_get_real_ns"
        | "ktime_get_coarse_real_ts64" => time_stub,
        "ktime_get_real_ts64" | "ktime_get_ts64" | "getnstimeofday64" => ktime_get_real_ts64,
        "jiffies_to_msecs" | "msecs_to_jiffies" => jiffies_to_msecs,

        // --- randomness and identity ------------------------------------------------
        "get_random_bytes" | "get_random_bytes_arch" => get_random_bytes,
        "get_random_u32" | "get_random_u64" | "get_random_int" | "prandom_u32" => random_stub,
        "capable" | "ns_capable" | "capable_wrt_inode_uidgid" => return_one,

        // --- IDR / xarray ------------------------------------------------------------
        "idr_alloc" | "idr_alloc_u32" | "ida_alloc" | "ida_alloc_range" => return_one,
        "idr_find" | "idr_remove" | "ida_free" | "idr_destroy" | "idr_preload"
        | "idr_preload_end" => return_zero,

        // --- compiler and hardening thunks -----------------------------------------
        // `__x86_return_thunk` is the retpoline replacement for `ret`, and
        // `__fentry__` is the ftrace hook at the top of every function. Both are
        // already fully handled by the gateway's return-address handling, so the
        // one thing they must not do is touch the return register.
        "__x86_return_thunk"
        | "__fentry__"
        | "mcount"
        | "__sanitizer_cov_trace_pc"
        | "__check_object_size"
        | "__ubsan_handle_load_invalid_value"
        | "__asan_report_load8_noabort" => ignore_call,

        // Hardened list/usercopy checks: they return "this operation is valid".
        // Answering false would make list_add() and friends silently skip the
        // operation, which would break the driver's own data structures.
        "__list_add_valid_or_report"
        | "__list_del_entry_valid_or_report"
        | "validate_usercopy_range"
        | "__list_valid_slowpath" => return_one,

        "__fortify_panic" | "__fortify_report" => fortify_abort,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn queue_work_on(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let work = if symbol == "queue_work_on" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        api_try!(emu, call.arg(emu, 0))
    };
    let func = emu.maps.read_qword(work + 0x18).unwrap_or(0);
    emu.kernel_defer(DeferredCall {
        kind: "work".to_string(),
        func,
        args: vec![work],
    });
    call.set_return(emu, 1);
}

fn queue_delayed_work_on(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let work = if symbol == "queue_delayed_work_on" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        api_try!(emu, call.arg(emu, 0))
    };
    let func = emu.maps.read_qword(work + 0x18).unwrap_or(0);
    emu.kernel_defer(DeferredCall {
        kind: "delayed_work".to_string(),
        func,
        args: vec![work],
    });
    call.set_return(emu, 1);
}

fn drain_work_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    // A cancel that actually cancels is the *fixed* version of the bug
    // class; draining here keeps a queued callback observable either way.
    emu.kernel_run_deferred();
    call.set_return(emu, 0);
}

fn workqueue_stub(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = emu.kernel_alloc(
        crate::kernel::heap::Region::Slab,
        0x100,
        "workqueue_struct",
        symbol,
        true,
    );
    call.set_return(emu, ptr);
}

fn mod_timer(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let timer = api_try!(emu, call.arg(emu, 0));
    let func = emu.maps.read_qword(timer + 0x18).unwrap_or(0);
    emu.kernel_defer(DeferredCall {
        kind: "timer".to_string(),
        func,
        args: vec![timer],
    });
    call.set_return(emu, 0);
}

fn cancel_timer_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    emu.kernel_run_deferred();
    call.set_return(emu, 1);
}

fn call_rcu(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let head = api_try!(emu, call.arg(emu, 0));
    let func = api_try!(emu, call.arg(emu, 1));
    emu.kernel_defer(DeferredCall {
        kind: "rcu".to_string(),
        func,
        args: vec![head],
    });
    call.set_return(emu, 0);
}

fn kvfree_call_rcu(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    // kfree_rcu(ptr, rcu_field) lowers to this; the object base is the
    // second argument on modern kernels.
    let ptr = api_try!(emu, call.arg(emu, 1));
    emu.kernel_free(ptr, "kfree_rcu");
    call.set_return(emu, 0);
}

fn task_stub(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = emu.kernel_alloc(
        crate::kernel::heap::Region::Slab,
        0x100,
        "task_struct",
        symbol,
        true,
    );
    call.set_return(emu, ptr);
}

fn delay_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    emu.tick += 1;
    call.set_return(emu, 0);
}

fn time_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    emu.tick += 1;
    call.set_return(emu, (emu.tick as u64) * 1_000_000);
}

fn ktime_get_real_ts64(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let ts = api_try!(emu, call.arg(emu, 0));
    emu.maps.write_qword(ts, emu.tick as u64);
    emu.maps.write_qword(ts + 8, 0);
    call.set_return(emu, 0);
}

fn jiffies_to_msecs(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let v = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, v);
}

fn get_random_bytes(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let buf = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let rip = emu.pc();
    emu.kernel_guard_access(rip, buf, len as u32, true);
    // Deterministic filler: an emulation that reproduces is worth more
    // than one that is unpredictable.
    emu.maps.write_bytes(buf, &vec![0x41u8; len as usize]);
    call.set_return(emu, 0);
}

fn random_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    call.set_return(emu, 0x41414141);
}

fn fortify_abort(emu: &mut Emu, _symbol: &str) {
    emu.kernel_log_line("detected buffer overflow in fortified helper".to_string());
    emu.stop();
}
