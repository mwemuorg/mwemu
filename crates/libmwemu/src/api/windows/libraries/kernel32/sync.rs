use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::common::memory::{read_pointer, writable_pointer, write_pointer};
use crate::winapi::policy::{self, api_try};

// An SRW lock with no waiters stores the owned bit and the count of shared owners.
const SRW_OWNED: u64 = 1;
const SRW_SHARED_OWNER: u64 = 1 << 4;

pub(crate) fn pointer_lock(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{library}!{api} object={address:#x}");
    if !writable_pointer(emu, address) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    if matches!(
        api,
        "InitializeConditionVariable" | "InitializeSRWLock" | "InitOnceInitialize"
    ) {
        write_pointer(emu, address, 0);
        call.cleanup_args(emu, 1);
        return;
    }
    let Some(state) = read_pointer(emu, address) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if matches!(api, "WakeConditionVariable" | "WakeAllConditionVariable") && state == 0 {
        // An initialized condition variable with no waiters needs no wakeup.
        call.cleanup_args(emu, 1);
        return;
    }
    let exclusive = api.ends_with("Exclusive");
    let shared = state & (SRW_SHARED_OWNER - 1) == SRW_OWNED && state >= SRW_SHARED_OWNER;
    let acquire = api.starts_with("Acquire") || api.starts_with("TryAcquire");
    let mut acquired = false;
    let next = if acquire && (state == 0 || (!exclusive && shared)) {
        acquired = true;
        if exclusive {
            Some(SRW_OWNED)
        } else {
            (state | SRW_OWNED).checked_add(SRW_SHARED_OWNER)
        }
    } else if api.starts_with("TryAcquire") {
        Some(state)
    } else if api == "ReleaseSRWLockExclusive" && state == SRW_OWNED {
        Some(0)
    } else if api == "ReleaseSRWLockShared" && shared {
        Some(if state == SRW_SHARED_OWNER | SRW_OWNED {
            0
        } else {
            state - SRW_SHARED_OWNER
        })
    } else {
        None
    };
    let Some(next) = next.filter(|next| emu.cfg.is_x64() || *next <= u32::MAX as u64) else {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    };
    if next != state {
        write_pointer(emu, address, next);
    }
    if api.starts_with("TryAcquire") {
        call.set_return(emu, u64::from(acquired));
    }
    call.cleanup_args(emu, 1);
}

pub(crate) fn interlocked(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    if api == "InterlockedCompareExchange64" {
        let exchange = api_try!(emu, call.arg_u64(emu, 1));
        let comparand = api_try!(emu, call.arg_u64(emu, if emu.cfg.is_x64() { 2 } else { 3 }));
        log_red!(
            emu,
            "{library}!{api} destination={address:#x} exchange={exchange:#x} comparand={comparand:#x}"
        );
        let Some(value) = emu.maps.read_qword(address) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        if !emu.maps.validate_write_range(address, 8)
            || (value == comparand && !emu.maps.write_qword(address, exchange))
        {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        call.set_return_u64(emu, value);
        call.cleanup_args(emu, 5);
    } else {
        let Some(original) = emu.maps.read_dword(address) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        let (next, returned, slots) = match api {
            "InterlockedCompareExchange" => {
                let exchange = api_try!(emu, call.arg_u32(emu, 1));
                let comparand = api_try!(emu, call.arg_u32(emu, 2));
                log_red!(
                    emu,
                    "{library}!{api} destination={address:#x} exchange={exchange:#x} comparand={comparand:#x}"
                );
                (
                    if original == comparand {
                        exchange
                    } else {
                        original
                    },
                    original,
                    3,
                )
            }
            "InterlockedExchange" | "InterlockedExchangeAdd" => {
                let value = api_try!(emu, call.arg_u32(emu, 1));
                log_red!(emu, "{library}!{api} target={address:#x} value={value:#x}");
                (
                    if api == "InterlockedExchange" {
                        value
                    } else {
                        original.wrapping_add(value)
                    },
                    original,
                    2,
                )
            }
            _ => {
                log_red!(emu, "{library}!{api} addend={address:#x}");
                let result = if api == "InterlockedIncrement" {
                    original.wrapping_add(1)
                } else {
                    original.wrapping_sub(1)
                };
                (result, result, 1)
            }
        };
        if !emu.maps.write_dword(address, next) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        call.set_return(emu, u64::from(returned));
        call.cleanup_args(emu, slots);
    }
}

pub(crate) fn sleep(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let millis = api_try!(emu, call.arg_u32(emu, 0));
    let extended = api == "SleepEx";
    let alertable = extended && api_try!(emu, call.arg_u32(emu, 1)) != 0;
    log_red!(
        emu,
        "{library}!{api} milliseconds={millis} alertable={alertable}"
    );
    if alertable || (millis == u32::MAX && !emu.cfg.short_circuit_sleep) {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    if emu.cfg.short_circuit_sleep || millis == 0 {
        emu.tick = emu.tick.saturating_add(1);
    } else {
        emu.tick = emu.tick.saturating_add(millis as usize);
        emu.current_thread_mut().wake_tick = emu.tick;
    }
    if extended {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 1 + usize::from(extended));
}

pub(crate) fn stack_probe(emu: &mut Emu, library: &str) {
    // This compiler helper receives its byte count in EAX/RAX. The x86 entry
    // also reserves the probed stack space; the x64 caller does that itself.
    let wide = emu.cfg.is_x64();
    let size = if wide {
        emu.regs().rax
    } else {
        emu.regs().get_eax()
    };
    log_red!(emu, "{library}!__chkstk size={size}");
    let Some(target) = emu.sp().checked_sub(size) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if size != 0 && !emu.maps.validate_write_range(target, size) {
        emu.exception(ExceptionType::BadAddressDereferencing);
    } else if !wide {
        emu.set_sp(target);
    }
}
