//! Time, randomness, capabilities and deferred execution.
//!
//! The deferred-work handlers are the interesting ones. Work items, timers and
//! RCU callbacks are how real drivers postpone a free — "unregister now, free
//! later" is the shape of most kernel use-after-free bugs. Running the callback
//! immediately would hide the window; queueing it and letting the caller decide
//! when to drain (see [`Emu::kernel_run_deferred`]) keeps the window visible.

use crate::emu::Emu;
use crate::kernel::DeferredCall;

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- deferred work ----------------------------------------------------
        // struct work_struct { atomic_long_t data; struct list_head entry;
        //                      work_func_t func; }  -> func at offset 0x18
        "queue_work_on" | "schedule_work" | "queue_work" => {
            let work = if symbol == "queue_work_on" {
                emu.kernel_arg(2)
            } else {
                emu.kernel_arg(0)
            };
            let func = emu.maps.read_qword(work + 0x18).unwrap_or(0);
            emu.kernel_defer(DeferredCall {
                kind: "work".to_string(),
                func,
                arg: work,
            });
            emu.set_kernel_ret(1);
        }
        "queue_delayed_work_on" | "schedule_delayed_work" => {
            let work = if symbol == "queue_delayed_work_on" {
                emu.kernel_arg(2)
            } else {
                emu.kernel_arg(0)
            };
            let func = emu.maps.read_qword(work + 0x18).unwrap_or(0);
            emu.kernel_defer(DeferredCall {
                kind: "delayed_work".to_string(),
                func,
                arg: work,
            });
            emu.set_kernel_ret(1);
        }
        "flush_work"
        | "cancel_work_sync"
        | "cancel_delayed_work"
        | "cancel_delayed_work_sync"
        | "flush_scheduled_work"
        | "flush_workqueue"
        | "destroy_workqueue"
        | "__flush_workqueue" => {
            // A cancel that actually cancels is the *fixed* version of the bug
            // class; draining here keeps a queued callback observable either way.
            emu.kernel_run_deferred();
            emu.set_kernel_ret(0);
        }
        "alloc_workqueue" | "__alloc_workqueue" | "__alloc_workqueue_key" => {
            let ptr = emu.kernel_alloc(
                crate::kernel::heap::Region::Slab,
                0x100,
                "workqueue_struct",
                symbol,
                true,
            );
            emu.set_kernel_ret(ptr);
        }
        "__init_work"
        | "__INIT_WORK"
        | "init_timer_key"
        | "timer_setup"
        | "__init_timer"
        | "__init_timer_on_stack" => emu.set_kernel_ret(0),

        // struct timer_list { ...; void (*function)(struct timer_list *); }
        // function sits at offset 0x18 on x86_64.
        "mod_timer" | "add_timer" | "timer_reduce" => {
            let timer = emu.kernel_arg(0);
            let func = emu.maps.read_qword(timer + 0x18).unwrap_or(0);
            emu.kernel_defer(DeferredCall {
                kind: "timer".to_string(),
                func,
                arg: timer,
            });
            emu.set_kernel_ret(0);
        }
        "del_timer"
        | "del_timer_sync"
        | "timer_delete"
        | "timer_delete_sync"
        | "timer_shutdown_sync" => {
            emu.kernel_run_deferred();
            emu.set_kernel_ret(1);
        }
        // call_rcu(head, func): func receives the rcu_head pointer.
        "call_rcu" | "call_rcu_hurry" => {
            let head = emu.kernel_arg(0);
            let func = emu.kernel_arg(1);
            emu.kernel_defer(DeferredCall {
                kind: "rcu".to_string(),
                func,
                arg: head,
            });
            emu.set_kernel_ret(0);
        }
        "kvfree_call_rcu" | "kfree_call_rcu" => {
            // kfree_rcu(ptr, rcu_field) lowers to this; the object base is the
            // second argument on modern kernels.
            let ptr = emu.kernel_arg(1);
            emu.kernel_free(ptr, "kfree_rcu");
            emu.set_kernel_ret(0);
        }

        // --- kthreads -----------------------------------------------------------
        "kthread_create_on_node" | "kthread_create" | "kthread_run" => {
            let ptr = emu.kernel_alloc(
                crate::kernel::heap::Region::Slab,
                0x100,
                "task_struct",
                symbol,
                true,
            );
            emu.set_kernel_ret(ptr);
        }
        "wake_up_process" | "kthread_stop" | "kthread_should_stop" => emu.set_kernel_ret(0),

        // --- time and delays ------------------------------------------------------
        "msleep"
        | "msleep_interruptible"
        | "ssleep"
        | "usleep_range"
        | "usleep_range_state"
        | "__const_udelay"
        | "__udelay"
        | "__ndelay"
        | "fsleep" => {
            emu.tick += 1;
            emu.set_kernel_ret(0);
        }
        "ktime_get"
        | "ktime_get_real"
        | "ktime_get_boottime"
        | "ktime_get_ns"
        | "ktime_get_real_ns"
        | "ktime_get_coarse_real_ts64" => {
            emu.tick += 1;
            emu.set_kernel_ret((emu.tick as u64) * 1_000_000);
        }
        "ktime_get_real_ts64" | "ktime_get_ts64" | "getnstimeofday64" => {
            let ts = emu.kernel_arg(0);
            emu.maps.write_qword(ts, emu.tick as u64);
            emu.maps.write_qword(ts + 8, 0);
            emu.set_kernel_ret(0);
        }
        "jiffies_to_msecs" | "msecs_to_jiffies" => {
            let v = emu.kernel_arg(0);
            emu.set_kernel_ret(v);
        }

        // --- randomness and identity ------------------------------------------------
        "get_random_bytes" | "get_random_bytes_arch" => {
            let buf = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, buf, len as u32, true);
            // Deterministic filler: an emulation that reproduces is worth more
            // than one that is unpredictable.
            emu.maps.write_bytes(buf, &vec![0x41u8; len as usize]);
            emu.set_kernel_ret(0);
        }
        "get_random_u32" | "get_random_u64" | "get_random_int" | "prandom_u32" => {
            emu.set_kernel_ret(0x41414141);
        }
        "capable" | "ns_capable" | "capable_wrt_inode_uidgid" => emu.set_kernel_ret(1),

        // --- IDR / xarray ------------------------------------------------------------
        "idr_alloc" | "idr_alloc_u32" | "ida_alloc" | "ida_alloc_range" => emu.set_kernel_ret(1),
        "idr_find" | "idr_remove" | "ida_free" | "idr_destroy" | "idr_preload"
        | "idr_preload_end" => emu.set_kernel_ret(0),

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
        | "__asan_report_load8_noabort" => {}

        // Hardened list/usercopy checks: they return "this operation is valid".
        // Answering false would make list_add() and friends silently skip the
        // operation, which would break the driver's own data structures.
        "__list_add_valid_or_report"
        | "__list_del_entry_valid_or_report"
        | "validate_usercopy_range"
        | "__list_valid_slowpath" => emu.set_kernel_ret(1),

        "__fortify_panic" | "__fortify_report" => {
            emu.kernel_log_line("detected buffer overflow in fortified helper".to_string());
            emu.stop();
        }

        _ => return false,
    }
    true
}
