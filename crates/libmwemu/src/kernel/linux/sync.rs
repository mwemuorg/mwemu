//! Locking, atomics and reference counting.
//!
//! Locks are modelled as no-ops on purpose: a single-threaded emulation cannot
//! deadlock, and pretending otherwise would only stop drivers from running.
//! Reference counts are the opposite — they are modelled for real, because in
//! a driver the refcount *is* the object lifetime. `refcount_dec_and_test()`
//! returning true is what triggers the free, so getting its arithmetic right is
//! what makes a use-after-free reproduce at all.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};

/// Read a `refcount_t` / `atomic_t` (a 32-bit counter behind a pointer).
fn read_counter(emu: &mut Emu, ptr: u64) -> i32 {
    let rip = emu.pc();
    emu.kernel_guard_access(rip, ptr, 4, false);
    emu.maps.read_dword(ptr).unwrap_or(0) as i32
}

fn write_counter(emu: &mut Emu, ptr: u64, value: i32) {
    let rip = emu.pc();
    emu.kernel_guard_access(rip, ptr, 4, true);
    emu.maps.write_dword(ptr, value as u32);
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- reference counting ---------------------------------------------
        "refcount_inc" | "refcount_inc_checked" | "__refcount_inc" => refcount_inc,
        "refcount_add" | "__refcount_add" => refcount_add,
        "refcount_dec" | "__refcount_dec" => refcount_dec,
        "refcount_dec_and_test" | "__refcount_dec_and_test" | "refcount_dec_and_test_checked" => refcount_dec_and_test,
        "refcount_sub_and_test" | "__refcount_sub_and_test" => refcount_sub_and_test,
        "refcount_inc_not_zero" | "__refcount_inc_not_zero" | "refcount_add_not_zero" => refcount_inc_not_zero,
        "refcount_warn_saturate" => refcount_warn_saturate,
        "kref_put" => kref_put,
        "kref_get" => refcount_inc,

        // --- mutexes, spinlocks, rwsems --------------------------------------
        // Single-threaded emulation: taking a lock cannot block and releasing
        // one cannot wake anybody, so these only need to succeed.
        "mutex_lock"
        | "mutex_unlock"
        | "__mutex_init"
        | "mutex_init_generic"
        | "mutex_lock_nested"
        | "mutex_lock_interruptible"
        | "mutex_lock_killable"
        | "mutex_destroy"
        | "down"
        | "up"
        | "down_read"
        | "up_read"
        | "down_write"
        | "up_write"
        | "__init_rwsem"
        | "down_read_killable"
        | "down_write_killable"
        | "_raw_spin_lock"
        | "_raw_spin_unlock"
        | "_raw_spin_lock_bh"
        | "_raw_spin_unlock_bh"
        | "_raw_spin_lock_irq"
        | "_raw_spin_unlock_irq"
        | "_raw_read_lock"
        | "_raw_read_unlock"
        | "_raw_write_lock"
        | "_raw_write_unlock"
        | "__raw_spin_lock_init"
        | "_raw_spin_lock_nested"
        | "queued_spin_lock_slowpath"
        | "local_bh_disable"
        | "local_bh_enable"
        | "preempt_count_add"
        | "preempt_count_sub"
        | "rcu_read_lock"
        | "rcu_read_unlock"
        | "synchronize_rcu"
        | "synchronize_rcu_expedited"
        | "rcu_barrier"
        | "might_resched"
        | "__might_sleep"
        | "__might_fault"
        | "__cond_resched"
        | "lock_acquire"
        | "lock_release" => return_zero,
        "mutex_trylock"
        | "mutex_trylock_nested"
        | "down_read_trylock"
        | "down_write_trylock"
        | "_raw_spin_trylock" => return_one,
        // irqsave variants return the saved flags; zero is a fine stand-in.
        "_raw_spin_lock_irqsave"
        | "_raw_spin_lock_irqsave_nested"
        | "_raw_read_lock_irqsave"
        | "_raw_write_lock_irqsave" => return_zero,
        "_raw_spin_unlock_irqrestore"
        | "_raw_read_unlock_irqrestore"
        | "_raw_write_unlock_irqrestore" => return_zero,

        // --- completions and wait queues ---------------------------------------
        "init_completion"
        | "complete"
        | "complete_all"
        | "wait_for_completion"
        | "wait_for_completion_interruptible"
        | "wait_for_completion_timeout"
        | "__init_waitqueue_head"
        | "__wake_up"
        | "prepare_to_wait_event"
        | "finish_wait"
        | "init_wait_entry"
        | "schedule"
        | "io_schedule" => return_zero,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn refcount_inc(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let v = read_counter(emu, p);
    write_counter(emu, p, v.wrapping_add(1));
    call.set_return(emu, 0);
}

fn refcount_add(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let n = api_try!(emu, call.arg_i32(emu, 0));
    let p = api_try!(emu, call.arg(emu, 1));
    let v = read_counter(emu, p);
    write_counter(emu, p, v.wrapping_add(n));
    call.set_return(emu, 0);
}

fn refcount_dec(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let v = read_counter(emu, p);
    write_counter(emu, p, v.wrapping_sub(1));
    call.set_return(emu, 0);
}

fn refcount_dec_and_test(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let v = read_counter(emu, p).wrapping_sub(1);
    write_counter(emu, p, v);
    call.set_return(emu, (v == 0) as u64);
}

fn refcount_sub_and_test(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let n = api_try!(emu, call.arg_i32(emu, 0));
    let p = api_try!(emu, call.arg(emu, 1));
    let v = read_counter(emu, p).wrapping_sub(n);
    write_counter(emu, p, v);
    call.set_return(emu, (v == 0) as u64);
}

fn refcount_inc_not_zero(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let v = read_counter(emu, p);
    if v == 0 {
        call.set_return(emu, 0);
    } else {
        write_counter(emu, p, v.wrapping_add(1));
        call.set_return(emu, 1);
    }
}

fn refcount_warn_saturate(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    emu.kernel_log_line("refcount_t: saturated; leaking memory".to_string());
    call.set_return(emu, 0);
}

fn kref_put(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    // kref_put(kref, release): drop the count, call release at zero.
    let kref = api_try!(emu, call.arg(emu, 0));
    let release = api_try!(emu, call.arg(emu, 1));
    let v = read_counter(emu, kref).wrapping_sub(1);
    write_counter(emu, kref, v);
    if v == 0 && release != 0 {
        let _ = emu.kernel_call(release, &[kref]);
    }
    call.set_return(emu, (v == 0) as u64);
}
