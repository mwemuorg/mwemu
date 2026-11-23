use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::crt::{self, Runtime};
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::crt::*;
use crate::windows::constants::errors::STATUS_ACCESS_VIOLATION;
use crate::windows::constants::exceptions::{
    EXCEPTION_CONTINUE_EXECUTION32, EXCEPTION_CONTINUE_SEARCH, EXCEPTION_EXECUTE_HANDLER,
};

fn state(emu: &mut Emu, create: bool) -> Option<u64> {
    let name = format!("ucrt_signal_state_{:x}", emu.current_thread().id);
    if let Some(map) = emu.maps.get_map_by_name(&name) {
        return Some(map.get_base());
    }
    if !create {
        return None;
    }
    let width = pointer_size(emu);
    emu.maps
        .alloc_rw_pages(&name, 2 * width)
        .map(|map| map.get_base())
}

pub(super) fn dispatch(emu: &mut Emu, api: &str) -> bool {
    match api {
        "signal" => register(emu),
        "_seh_filter_exe" => filter(emu),
        "__pxcptinfoptrs" => {
            let address = state(emu, true)
                .map(|base| base + pointer_size(emu))
                .unwrap_or(0);
            WinApi(Cdecl).set_return(emu, address);
        }
        _ => return false,
    }
    true
}

fn register(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let [signal, action] = api_try!(emu, call.args(emu));
    log_red!(emu, "ucrtbase!signal signal={signal} action={action:#x}");
    if signal as u32 != SIGSEGV {
        policy::unsupported_with_abi(emu, "ucrtbase", "signal", emu.pc(), call);
        return;
    }
    let failure = if pointer_size(emu) == 4 {
        u64::from(u32::MAX)
    } else {
        u64::MAX
    };
    if matches!(action, SIG_SGE | SIG_ACK) {
        if crt::record_errno(emu, Runtime::Ucrt, EINVAL) {
            call.set_return(emu, failure);
        }
        return;
    }
    let Some(base) = state(emu, true) else {
        if crt::record_errno(emu, Runtime::Ucrt, ENOMEM) {
            call.set_return(emu, failure);
        }
        return;
    };
    let Some(previous) = read(emu, base) else {
        return;
    };
    if action != SIG_GET && !write(emu, base, action) {
        return;
    }
    call.set_return(emu, previous);
}

fn read(emu: &mut Emu, addr: u64) -> Option<u64> {
    let value = read_pointer(emu, addr);
    if value.is_none() {
        emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            addr,
            pointer_size(emu) as usize,
            MemoryAccess::Read,
        );
    }
    value
}

fn write(emu: &mut Emu, addr: u64, value: u64) -> bool {
    if write_pointer(emu, addr, value) {
        return true;
    }
    emu.memory_exception(
        ExceptionType::BadAddressDereferencing,
        addr,
        pointer_size(emu) as usize,
        MemoryAccess::Write,
    );
    false
}

fn filter(emu: &mut Emu) {
    const API: &str = "_seh_filter_exe";
    let call = WinApi(Cdecl);
    let [code, pointers] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "ucrtbase!{API} exception_code={code:#x} exception_pointers={pointers:#x}"
    );
    if u64::from(code as u32) != STATUS_ACCESS_VIOLATION {
        policy::unsupported_with_abi(emu, "ucrtbase", API, emu.pc(), call);
        return;
    }
    // UCRT continues searching when it cannot obtain per-thread exception state.
    let Some(base) = state(emu, false) else {
        call.set_return(emu, u64::from(EXCEPTION_CONTINUE_SEARCH));
        return;
    };
    let Some(action) = read(emu, base) else {
        return;
    };
    if action == SIG_DFL {
        call.set_return(emu, u64::from(EXCEPTION_CONTINUE_SEARCH));
        return;
    }
    if action != SIG_IGN {
        if !write(emu, base, SIG_DFL) {
            return;
        }
        if action == SIG_DIE {
            call.set_return(emu, u64::from(EXCEPTION_EXECUTE_HANDLER));
            return;
        }
        let slot = base + pointer_size(emu);
        let Some(previous) = read(emu, slot) else {
            return;
        };
        if !write(emu, slot, pointers) {
            return;
        }
        if policy::invoke_callback(emu, "ucrtbase", API, action, [u64::from(SIGSEGV)]).is_none() {
            return;
        }
        if !write(emu, slot, previous) {
            return;
        }
    }
    call.set_return(emu, u64::from(EXCEPTION_CONTINUE_EXECUTION32));
}
