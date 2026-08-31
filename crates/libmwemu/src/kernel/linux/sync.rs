//! Locking, atomics and reference counting.
//!
//! Locks are modelled as no-ops on purpose: a single-threaded emulation cannot
//! deadlock, and pretending otherwise would only stop drivers from running.
//! Reference counts are the opposite — they are modelled for real, because in
//! a driver the refcount *is* the object lifetime. `refcount_dec_and_test()`
//! returning true is what triggers the free, so getting its arithmetic right is
//! what makes a use-after-free reproduce at all.

use crate::emu::Emu;

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
    match symbol {
        // --- reference counting ---------------------------------------------
        "refcount_inc" | "refcount_inc_checked" | "__refcount_inc" => {
            let p = emu.kernel_arg(0);
            let v = read_counter(emu, p);
            write_counter(emu, p, v.wrapping_add(1));
            emu.set_kernel_ret(0);
        }
        "refcount_add" | "__refcount_add" => {
            let n = emu.kernel_arg(0) as i32;
            let p = emu.kernel_arg(1);
            let v = read_counter(emu, p);
            write_counter(emu, p, v.wrapping_add(n));
            emu.set_kernel_ret(0);
        }
        "refcount_dec" | "__refcount_dec" => {
            let p = emu.kernel_arg(0);
            let v = read_counter(emu, p);
            write_counter(emu, p, v.wrapping_sub(1));
            emu.set_kernel_ret(0);
        }
        "refcount_dec_and_test" | "__refcount_dec_and_test" | "refcount_dec_and_test_checked" => {
            let p = emu.kernel_arg(0);
            let v = read_counter(emu, p).wrapping_sub(1);
            write_counter(emu, p, v);
            emu.set_kernel_ret((v == 0) as u64);
        }
        "refcount_sub_and_test" | "__refcount_sub_and_test" => {
            let n = emu.kernel_arg(0) as i32;
            let p = emu.kernel_arg(1);
            let v = read_counter(emu, p).wrapping_sub(n);
            write_counter(emu, p, v);
            emu.set_kernel_ret((v == 0) as u64);
        }
        "refcount_inc_not_zero" | "__refcount_inc_not_zero" | "refcount_add_not_zero" => {
            let p = emu.kernel_arg(0);
            let v = read_counter(emu, p);
            if v == 0 {
                emu.set_kernel_ret(0);
            } else {
                write_counter(emu, p, v.wrapping_add(1));
                emu.set_kernel_ret(1);
            }
        }
        "refcount_warn_saturate" => {
            emu.kernel_log_line("refcount_t: saturated; leaking memory".to_string());
            emu.set_kernel_ret(0);
        }
        "kref_put" => {
            // kref_put(kref, release): drop the count, call release at zero.
            let kref = emu.kernel_arg(0);
            let release = emu.kernel_arg(1);
            let v = read_counter(emu, kref).wrapping_sub(1);
            write_counter(emu, kref, v);
            if v == 0 && release != 0 {
                let _ = emu.kernel_call(release, &[kref]);
            }
            emu.set_kernel_ret((v == 0) as u64);
        }
        "kref_get" => {
            let p = emu.kernel_arg(0);
            let v = read_counter(emu, p);
            write_counter(emu, p, v.wrapping_add(1));
            emu.set_kernel_ret(0);
        }

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
        | "lock_release" => {
            emu.set_kernel_ret(0);
        }
        "mutex_trylock"
        | "mutex_trylock_nested"
        | "down_read_trylock"
        | "down_write_trylock"
        | "_raw_spin_trylock" => {
            emu.set_kernel_ret(1); // always acquired
        }
        // irqsave variants return the saved flags; zero is a fine stand-in.
        "_raw_spin_lock_irqsave"
        | "_raw_spin_lock_irqsave_nested"
        | "_raw_read_lock_irqsave"
        | "_raw_write_lock_irqsave" => {
            emu.set_kernel_ret(0);
        }
        "_raw_spin_unlock_irqrestore"
        | "_raw_read_unlock_irqrestore"
        | "_raw_write_unlock_irqrestore" => {
            emu.set_kernel_ret(0);
        }

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
        | "io_schedule" => {
            emu.set_kernel_ret(0);
        }

        _ => return false,
    }
    true
}
