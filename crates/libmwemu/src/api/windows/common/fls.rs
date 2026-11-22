use std::mem;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::threading::context::ArchThreadState;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::{
    ERROR_INVALID_PARAMETER, ERROR_NO_MORE_ITEMS, ERROR_SUCCESS,
};
use crate::windows::constants::process::{FLS_MAXIMUM_AVAILABLE, FLS_OUT_OF_INDEXES};

#[derive(Debug, Default)]
pub(crate) struct FlsCleanup {
    freeing: Vec<usize>,
    detaching: Vec<usize>,
}

fn allocate(emu: &mut Emu, callback: u64) -> u64 {
    let slots = &mut emu.windows_fls_slots;
    if slots.is_empty() {
        slots.push(None);
    }
    let index = slots
        .iter()
        .skip(1)
        .position(Option::is_none)
        .map_or(slots.len(), |n| n + 1);
    if index >= FLS_MAXIMUM_AVAILABLE {
        emu.set_last_error(ERROR_NO_MORE_ITEMS);
        return u64::from(FLS_OUT_OF_INDEXES);
    }
    if index == slots.len() {
        slots.push(Some(callback));
    } else {
        slots[index] = Some(callback);
    }
    index as u64
}

fn take_value(emu: &mut Emu, thread: usize, index: usize) -> u64 {
    match &mut emu.threads[thread].arch {
        ArchThreadState::X86 { fls, .. } => fls.get_mut(index).map_or(0, mem::take),
        _ => 0,
    }
}

fn free(emu: &mut Emu, library: &str, index: usize, callback: u64) -> bool {
    if emu.fls_cleanup.freeing.contains(&index) {
        // Windows blocks if a destructor tries to free its own active index.
        policy::callback_failed(emu, library, "FlsFree");
        return false;
    }
    emu.fls_cleanup.freeing.push(index);
    let mut complete = true;
    'drain: loop {
        let mut invoked = false;
        for thread in 0..emu.threads.len() {
            let value = take_value(emu, thread, index);
            if value != 0 && callback != 0 {
                if policy::invoke_callback(emu, library, "FlsFree", callback, [value]).is_none() {
                    complete = false;
                    break 'drain;
                }
                invoked = true;
            }
        }
        // A callback can set the index again in the calling fiber.
        if !invoked {
            break;
        }
    }
    emu.fls_cleanup.freeing.pop();
    if complete {
        emu.windows_fls_slots[index] = None;
    }
    complete
}

/// Windows keeps values visible to the exiting fiber while running its destructors.
pub(crate) fn detach_current(emu: &mut Emu, api: &str) -> bool {
    let thread = emu.current_thread_id;
    if emu.fls_cleanup.detaching.contains(&thread) {
        return true;
    }
    emu.fls_cleanup.detaching.push(thread);
    let mut index = 1;
    let mut complete = true;
    while index < emu.fls().len() {
        let value = emu.fls()[index];
        let callback = emu
            .windows_fls_slots
            .get(index)
            .copied()
            .flatten()
            .unwrap_or(0);
        if value != 0
            && callback != 0
            && policy::invoke_callback(emu, "kernel32", api, callback, [value]).is_none()
        {
            complete = false;
            break;
        }
        index += 1;
    }
    emu.fls_cleanup.detaching.pop();
    if complete {
        emu.fls_mut().clear();
    }
    complete
}

pub(crate) fn dispatch(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    if api == "FlsAlloc" {
        let callback = api_try!(emu, call.arg(emu, 0));
        let result = allocate(emu, callback);
        log_red!(emu, "{library}!{api} callback={callback:#x} => {result:#x}");
        call.set_return(emu, result);
        call.cleanup_args(emu, 1);
        return;
    }
    let index = api_try!(emu, call.arg_u32(emu, 0)) as usize;
    let value = if api == "FlsSetValue" {
        Some(api_try!(emu, call.arg(emu, 1)))
    } else {
        None
    };
    let callback = emu.windows_fls_slots.get(index).copied().flatten();
    let result = match (callback, value) {
        (None, _) => {
            if api != "FlsGetValue2" {
                emu.set_last_error(ERROR_INVALID_PARAMETER);
            }
            0
        }
        (Some(_), Some(value)) => {
            let values = emu.fls_mut();
            if values.len() <= index {
                values.resize(index + 1, 0);
            }
            values[index] = value;
            1
        }
        (Some(callback), None) if api == "FlsFree" => {
            if !free(emu, library, index, callback) {
                return;
            }
            1
        }
        _ => {
            if api == "FlsGetValue" {
                emu.set_last_error(ERROR_SUCCESS);
            }
            emu.fls().get(index).copied().unwrap_or(0)
        }
    };
    if let Some(value) = value {
        log_red!(
            emu,
            "{library}!{api} index={index} value={value:#x} => {result:#x}"
        );
    } else {
        log_red!(emu, "{library}!{api} index={index} => {result:#x}");
    }
    call.set_return(emu, result);
    call.cleanup_args(emu, if value.is_some() { 2 } else { 1 });
}
