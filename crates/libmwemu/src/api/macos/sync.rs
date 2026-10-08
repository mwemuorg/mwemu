//! Thread synchronisation: pthread mutexes (normal / errorcheck / recursive),
//! condition variables and `os_unfair_lock`.
//!
//! Lock state lives in the guest's own lock object, so `fork()` copies it
//! with the rest of memory. A thread that has to wait is parked with
//! `blocked_on_lock` / `cond_wait`; whoever releases the lock completes the
//! waiter's call (sets its return value and hands it ownership).
use super::libc_extra::trace;
use crate::api::abi::ApiAbi;
use crate::emu::Emu;

// pthread_mutex_t: sig at +0 (from <pthread/pthread_impl.h>), then our state.
const SIG_MUTEX: u64 = 0x32AA_ABA7;
const SIG_ERRORCHECK: u64 = 0x32AA_ABA1;
const SIG_RECURSIVE: u64 = 0x32AA_ABA2;
const OWNER: u64 = 8; // u64 thread id, 0 = unlocked
const COUNT: u64 = 16; // u32 recursion depth
const KIND: u64 = 20; // u32 type set by pthread_mutex_init
const ATTR_KIND: u64 = 8; // pthread_mutexattr_t: u32 type after the sig

const NORMAL: u32 = 0;
const ERRORCHECK: u32 = 1;
const RECURSIVE: u32 = 2;

const EPERM: u64 = 1;
const EDEADLK: u64 = 11;
const EBUSY: u64 = 16;
const ETIMEDOUT: u64 = 60;

/// Dispatch `symbol` if it is implemented here. Returns false otherwise.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let name = symbol.strip_prefix('_').unwrap_or(symbol);
    match name {
        "pthread_mutex_init" => api_mutex_init(emu),
        "pthread_mutex_lock" => api_mutex_lock(emu),
        "pthread_mutex_trylock" => api_mutex_trylock(emu),
        "pthread_mutex_unlock" => api_mutex_unlock(emu),
        "pthread_mutex_destroy" => api_mutex_destroy(emu),
        "pthread_mutexattr_init" => api_mutexattr_init(emu),
        "pthread_mutexattr_settype" => api_mutexattr_settype(emu),
        "pthread_mutexattr_gettype" => api_mutexattr_gettype(emu),
        "pthread_mutexattr_destroy" | "pthread_cond_init" | "pthread_cond_destroy" => {
            trace(emu, &format!("{}() -> 0", name));
            ApiAbi::from_emu(emu).set_ret(emu, 0);
        }
        "pthread_cond_wait" => api_cond_wait(emu),
        "pthread_cond_timedwait" => api_cond_timedwait(emu),
        "pthread_cond_signal" => api_cond_wake(emu, false),
        "pthread_cond_broadcast" => api_cond_wake(emu, true),
        "os_unfair_lock_lock" | "os_unfair_lock_lock_with_options" => api_unfair_lock(emu),
        "os_unfair_lock_trylock" => api_unfair_trylock(emu),
        "os_unfair_lock_unlock" => api_unfair_unlock(emu),
        "os_unfair_lock_assert_owner" | "os_unfair_lock_assert_not_owner" => {}
        _ => return false,
    }
    true
}

fn current_tid(emu: &Emu) -> u64 {
    emu.threads[emu.current_thread_id].id
}

/// Next thread (round-robin after the current one) waiting to own `addr`.
fn next_waiter(emu: &Emu, addr: u64) -> Option<usize> {
    let n = emu.threads.len();
    (1..=n)
        .map(|i| (emu.current_thread_id + i) % n)
        .find(|&i| emu.threads[i].blocked_on_lock == Some(addr))
}

/// Park the current thread until a release hands it `addr`.
fn block_on_lock(emu: &mut Emu, name: &str, addr: u64) {
    trace(emu, &format!("{}(0x{:x}) -> blocking", name, addr));
    let cur = emu.current_thread_id;
    emu.threads[cur].blocked_on_lock = Some(addr);
}

/// Complete a parked thread's lock call: it now runs with return value 0.
fn wake(emu: &mut Emu, idx: usize) {
    emu.threads[idx].blocked_on_lock = None;
    emu.threads[idx].regs_aarch64_mut().x[0] = 0;
}

// --- pthread_mutex -------------------------------------------------------

fn mutex_kind(emu: &Emu, m: u64) -> u32 {
    match emu.maps.read_qword(m).unwrap_or(0) & 0xffff_ffff {
        SIG_RECURSIVE => RECURSIVE,
        SIG_ERRORCHECK => ERRORCHECK,
        _ => emu.maps.read_dword(m + KIND).unwrap_or(NORMAL),
    }
}

fn mutex_owner(emu: &Emu, m: u64) -> u64 {
    emu.maps.read_qword(m + OWNER).unwrap_or(0)
}

fn mutex_take(emu: &mut Emu, m: u64, tid: u64) {
    emu.maps.write_qword(m + OWNER, tid);
    emu.maps.write_dword(m + COUNT, 1);
}

/// Fully release `m`, handing it to the next waiter if there is one.
fn mutex_release(emu: &mut Emu, m: u64) {
    match next_waiter(emu, m) {
        Some(idx) => {
            let tid = emu.threads[idx].id;
            mutex_take(emu, m, tid);
            wake(emu, idx);
        }
        None => {
            emu.maps.write_qword(m + OWNER, 0);
            emu.maps.write_dword(m + COUNT, 0);
        }
    }
}

fn api_mutex_init(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let m = abi.arg(emu, 0);
    let attr = abi.arg(emu, 1);
    let kind = if attr == 0 {
        NORMAL
    } else {
        emu.maps.read_dword(attr + ATTR_KIND).unwrap_or(NORMAL)
    };
    emu.maps.write_qword(m, SIG_MUTEX);
    emu.maps.write_qword(m + OWNER, 0);
    emu.maps.write_dword(m + COUNT, 0);
    emu.maps.write_dword(m + KIND, kind);
    trace(
        emu,
        &format!("pthread_mutex_init(0x{:x}, type={}) -> 0", m, kind),
    );
    abi.set_ret(emu, 0);
}

fn api_mutex_lock(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let m = abi.arg(emu, 0);
    let tid = current_tid(emu);
    let owner = mutex_owner(emu, m);

    if owner == 0 {
        mutex_take(emu, m, tid);
        trace(emu, &format!("pthread_mutex_lock(0x{:x}) -> 0", m));
        abi.set_ret(emu, 0);
        return;
    }
    if owner == tid {
        match mutex_kind(emu, m) {
            RECURSIVE => {
                let count = emu.maps.read_dword(m + COUNT).unwrap_or(0);
                emu.maps.write_dword(m + COUNT, count + 1);
                abi.set_ret(emu, 0);
                return;
            }
            ERRORCHECK => {
                trace(emu, &format!("pthread_mutex_lock(0x{:x}) -> EDEADLK", m));
                abi.set_ret(emu, EDEADLK);
                return;
            }
            _ => {} // a normal mutex relocked by its owner deadlocks, as natively
        }
    }
    block_on_lock(emu, "pthread_mutex_lock", m);
}

fn api_mutex_trylock(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let m = abi.arg(emu, 0);
    let tid = current_tid(emu);
    let owner = mutex_owner(emu, m);

    let ret = if owner == 0 {
        mutex_take(emu, m, tid);
        0
    } else if owner == tid && mutex_kind(emu, m) == RECURSIVE {
        let count = emu.maps.read_dword(m + COUNT).unwrap_or(0);
        emu.maps.write_dword(m + COUNT, count + 1);
        0
    } else {
        EBUSY
    };
    trace(emu, &format!("pthread_mutex_trylock(0x{:x}) -> {}", m, ret));
    abi.set_ret(emu, ret);
}

fn api_mutex_unlock(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let m = abi.arg(emu, 0);
    let tid = current_tid(emu);

    if mutex_owner(emu, m) != tid && mutex_kind(emu, m) != NORMAL {
        trace(emu, &format!("pthread_mutex_unlock(0x{:x}) -> EPERM", m));
        abi.set_ret(emu, EPERM);
        return;
    }
    let count = emu.maps.read_dword(m + COUNT).unwrap_or(1);
    if count > 1 {
        emu.maps.write_dword(m + COUNT, count - 1);
    } else {
        mutex_release(emu, m);
    }
    trace(emu, &format!("pthread_mutex_unlock(0x{:x}) -> 0", m));
    abi.set_ret(emu, 0);
}

fn api_mutex_destroy(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let m = abi.arg(emu, 0);
    let ret = if mutex_owner(emu, m) == 0 { 0 } else { EBUSY };
    trace(emu, &format!("pthread_mutex_destroy(0x{:x}) -> {}", m, ret));
    abi.set_ret(emu, ret);
}

fn api_mutexattr_init(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let attr = abi.arg(emu, 0);
    emu.maps.write_dword(attr + ATTR_KIND, NORMAL);
    trace(emu, &format!("pthread_mutexattr_init(0x{:x}) -> 0", attr));
    abi.set_ret(emu, 0);
}

fn api_mutexattr_settype(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let attr = abi.arg(emu, 0);
    let kind = abi.arg(emu, 1) as u32;
    emu.maps.write_dword(attr + ATTR_KIND, kind);
    trace(
        emu,
        &format!("pthread_mutexattr_settype(0x{:x}, {}) -> 0", attr, kind),
    );
    abi.set_ret(emu, 0);
}

fn api_mutexattr_gettype(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let attr = abi.arg(emu, 0);
    let out = abi.arg(emu, 1);
    let kind = emu.maps.read_dword(attr + ATTR_KIND).unwrap_or(NORMAL);
    emu.maps.write_dword(out, kind);
    abi.set_ret(emu, 0);
}

// --- pthread_cond --------------------------------------------------------

/// Release the caller's mutex for a condition wait. False if it is not held
/// by the caller (EPERM, nothing released).
fn release_for_wait(emu: &mut Emu, name: &str, m: u64) -> bool {
    if mutex_owner(emu, m) != current_tid(emu) {
        trace(emu, &format!("{}(mutex=0x{:x}) -> EPERM", name, m));
        ApiAbi::from_emu(emu).set_ret(emu, EPERM);
        return false;
    }
    mutex_release(emu, m);
    true
}

fn api_cond_wait(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let c = abi.arg(emu, 0);
    let m = abi.arg(emu, 1);
    if !release_for_wait(emu, "pthread_cond_wait", m) {
        return;
    }
    trace(emu, &format!("pthread_cond_wait(0x{:x}) -> blocking", c));
    let cur = emu.current_thread_id;
    emu.threads[cur].cond_wait = Some((c, m));
}

/// Timed wait: release, then retake the mutex. Returns ETIMEDOUT once the
/// host clock passes `abstime`, otherwise 0 (a spurious wakeup, which
/// callers must already handle), so it can never deadlock.
fn api_cond_timedwait(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let m = abi.arg(emu, 1);
    let abstime = abi.arg(emu, 2);
    if !release_for_wait(emu, "pthread_cond_timedwait", m) {
        return;
    }
    let deadline = emu.maps.read_qword(abstime).unwrap_or(0) as u128 * 1_000_000_000
        + emu.maps.read_qword(abstime + 8).unwrap_or(0) as u128;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let ret = if now >= deadline { ETIMEDOUT } else { 0 };
    trace(emu, &format!("pthread_cond_timedwait() -> {}", ret));

    let tid = current_tid(emu);
    if mutex_owner(emu, m) == 0 {
        mutex_take(emu, m, tid);
        abi.set_ret(emu, ret);
    } else {
        // ETIMEDOUT is lost if we have to wait for the mutex; 0 is still valid.
        block_on_lock(emu, "pthread_cond_timedwait", m);
    }
}

fn api_cond_wake(emu: &mut Emu, all: bool) {
    let abi = ApiAbi::from_emu(emu);
    let c = abi.arg(emu, 0);
    let mut woken = 0;
    while let Some(idx) = cond_waiter(emu, c) {
        resume_cond_waiter(emu, idx);
        woken += 1;
        if !all {
            break;
        }
    }
    let name = if all {
        "pthread_cond_broadcast"
    } else {
        "pthread_cond_signal"
    };
    trace(emu, &format!("{}(0x{:x}) -> 0 woke {}", name, c, woken));
    abi.set_ret(emu, 0);
}

fn cond_waiter(emu: &Emu, c: u64) -> Option<usize> {
    let n = emu.threads.len();
    (1..=n)
        .map(|i| (emu.current_thread_id + i) % n)
        .find(|&i| emu.threads[i].cond_wait.is_some_and(|(cond, _)| cond == c))
}

/// A signalled waiter must retake its mutex before its wait returns.
fn resume_cond_waiter(emu: &mut Emu, idx: usize) {
    let Some((_, m)) = emu.threads[idx].cond_wait.take() else {
        return;
    };
    if mutex_owner(emu, m) == 0 {
        let tid = emu.threads[idx].id;
        mutex_take(emu, m, tid);
        emu.threads[idx].regs_aarch64_mut().x[0] = 0;
    } else {
        emu.threads[idx].blocked_on_lock = Some(m);
    }
}

// --- os_unfair_lock (u32 owner, 0 = unlocked) ----------------------------

fn api_unfair_lock(emu: &mut Emu) {
    let lock = ApiAbi::from_emu(emu).arg(emu, 0);
    let tid = current_tid(emu) as u32;
    if emu.maps.read_dword(lock).unwrap_or(0) == 0 {
        emu.maps.write_dword(lock, tid);
        trace(emu, &format!("os_unfair_lock_lock(0x{:x})", lock));
        return;
    }
    // Relocking by the owner aborts natively; blocking surfaces it as a deadlock.
    block_on_lock(emu, "os_unfair_lock_lock", lock);
}

fn api_unfair_trylock(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let lock = abi.arg(emu, 0);
    let free = emu.maps.read_dword(lock).unwrap_or(0) == 0;
    if free {
        let tid = current_tid(emu) as u32;
        emu.maps.write_dword(lock, tid);
    }
    trace(
        emu,
        &format!("os_unfair_lock_trylock(0x{:x}) -> {}", lock, free),
    );
    abi.set_ret(emu, free as u64);
}

fn api_unfair_unlock(emu: &mut Emu) {
    let lock = ApiAbi::from_emu(emu).arg(emu, 0);
    match next_waiter(emu, lock) {
        Some(idx) => {
            let tid = emu.threads[idx].id as u32;
            emu.maps.write_dword(lock, tid);
            wake(emu, idx);
        }
        None => {
            emu.maps.write_dword(lock, 0);
        }
    }
    trace(emu, &format!("os_unfair_lock_unlock(0x{:x})", lock));
}
