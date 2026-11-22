use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;

pub(crate) fn critical_section(emu: &mut Emu, api: &str) {
    section(emu, "ntdll", api);
}

pub(crate) fn section(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let (words, initialize) = match api {
        "RtlInitializeCriticalSectionEx" | "InitializeCriticalSectionEx" => (3, true),
        "RtlInitializeCriticalSectionAndSpinCount" | "InitializeCriticalSectionAndSpinCount" => {
            (2, true)
        }
        "SetCriticalSectionSpinCount" => (2, false),
        "RtlInitializeCriticalSection" | "InitializeCriticalSection" => (1, true),
        _ => (1, false),
    };
    let spin = if words >= 2 {
        api_try!(emu, call.arg_u32(emu, 1))
    } else {
        0
    };
    let flags = if words == 3 {
        api_try!(emu, call.arg_u32(emu, 2))
    } else {
        0
    };
    let width = pointer_size(emu);
    let size = 4 * width + 8;
    log_red!(
        emu,
        "{library}!{api} section={ptr:#x} spin={spin} flags={flags:#x}"
    );
    let native = api.starts_with("Rtl");
    let mut result = None;
    let status = if ptr == 0 || !emu.maps.validate_write_range(ptr, size) {
        if !native {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        STATUS_ACCESS_VIOLATION
    } else if api == "SetCriticalSectionSpinCount" {
        let spin_address = ptr + size - width;
        result = read_pointer(emu, spin_address);
        write_pointer(emu, spin_address, u64::from(spin));
        STATUS_SUCCESS
    } else if initialize {
        if flags & !RTL_CRITICAL_SECTION_MODELED_FLAGS != 0 {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        } else {
            emu.global_locks.crit_map.remove(&ptr);
            emu.maps.fill_range(ptr, 0, size as usize);
            write_pointer(emu, ptr, u64::MAX);
            emu.maps.write_dword(ptr + width, u32::MAX);
            write_pointer(emu, ptr + size - width, u64::from(spin));
            STATUS_SUCCESS
        }
    } else if api == "RtlDeleteCriticalSection" || api == "DeleteCriticalSection" {
        emu.global_locks.crit_map.remove(&ptr);
        emu.maps.fill_range(ptr, 0, size as usize);
        STATUS_SUCCESS
    } else {
        let tid = emu.current_thread().id;
        if api == "TryEnterCriticalSection" {
            let available = emu
                .global_locks
                .crit_map
                .get(&ptr)
                .and_then(|cs| cs.owner_tid)
                .is_none_or(|owner| owner == tid);
            result = Some(u64::from(available && emu.global_locks.enter(ptr, tid)));
        } else if api == "RtlEnterCriticalSection" || api == "EnterCriticalSection" {
            if !emu.global_locks.enter(ptr, tid) {
                emu.current_thread_mut().blocked_on_cs = Some(ptr);
            }
        } else if let Some(next) = emu.global_locks.leave(ptr, tid)
            && let Some(thread) = emu.threads.iter_mut().find(|t| t.id == next)
        {
            thread.blocked_on_cs = None;
            thread.wake_tick = 0;
        }
        if let Some(state) = emu.global_locks.crit_map.get(&ptr) {
            let recursion = state.recursion_count as u32;
            let owner = state.owner_tid.unwrap_or(0);
            emu.maps.write_dword(ptr + width + 4, recursion);
            write_pointer(emu, ptr + width + 8, owner);
        }
        STATUS_SUCCESS
    };
    if native {
        call.set_return(emu, status);
    } else if let Some(value) = result {
        call.set_return(emu, value);
    } else if initialize && words > 1 {
        call.set_return(emu, u64::from(status == STATUS_SUCCESS));
    }
    call.cleanup_args(emu, words);
}

const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: u32 = 0x0100_0000;
const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: u32 = 0x0200_0000;
const RTL_CRITICAL_SECTION_MODELED_FLAGS: u32 =
    RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO | RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN;
