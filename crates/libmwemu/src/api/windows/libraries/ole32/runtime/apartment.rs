use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{
    E_OUTOFMEMORY, HRESULT_E_INVALID_ARG, RPC_E_CALL_COMPLETE, RPC_E_CHANGED_MODE, S_FALSE, S_OK,
};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Apartment {
    mode: u32,
    references: u32,
    ole_references: u32,
}

const COINIT_APARTMENTTHREADED: u32 = 2;
const COINIT_DISABLE_OLE1DDE: u32 = 4;
const COINIT_SPEED_OVER_MEMORY: u32 = 8;
const OLE_MAJOR_VERSION: u64 = 23;
const OLE_BUILD: u64 = 639;

fn initialize(emu: &mut Emu, api: &str, extended: bool, ole: bool) {
    let call = WinApi(Stdcall);
    let reserved = api_try!(emu, call.arg(emu, 0));
    let flags = if extended {
        api_try!(emu, call.arg_u32(emu, 1))
    } else {
        COINIT_APARTMENTTHREADED
    };
    let mode = flags & COINIT_APARTMENTTHREADED;
    let thread = emu.current_thread().id;
    let status = if reserved != 0
        || flags & !(COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE | COINIT_SPEED_OVER_MEMORY)
            != 0
    {
        HRESULT_E_INVALID_ARG
    } else {
        match emu.com_apartments.entry(thread) {
            Entry::Vacant(slot) => {
                slot.insert(Apartment {
                    mode,
                    references: 1,
                    ole_references: u32::from(ole),
                });
                S_OK
            }
            Entry::Occupied(mut slot) => {
                let apartment = slot.get_mut();
                if apartment.mode != mode {
                    RPC_E_CHANGED_MODE
                } else if let Some(n) = apartment.references.checked_add(1) {
                    apartment.references = n;
                    let first_ole = ole && apartment.ole_references == 0;
                    if ole {
                        apartment.ole_references += 1;
                    }
                    if first_ole { S_OK } else { S_FALSE }
                } else {
                    E_OUTOFMEMORY
                }
            }
        }
    };
    log_red!(
        emu,
        "ole32!{api} reserved={reserved:#x} flags={flags:#x} thread={thread} status={status:#x}"
    );
    call.cleanup_args(emu, 1 + usize::from(extended));
    call.set_return(emu, status);
}
fn uninitialize(emu: &mut Emu, api: &str, ole: bool) {
    let thread = emu.current_thread().id;
    if let Entry::Occupied(mut slot) = emu.com_apartments.entry(thread)
        && (!ole || slot.get().ole_references != 0)
    {
        if ole {
            slot.get_mut().ole_references -= 1;
        }
        if slot.get().references <= 1 {
            slot.remove();
        } else {
            slot.get_mut().references -= 1;
        }
    }
    log_red!(emu, "ole32!{api} thread={thread}");
}
pub fn OleInitialize(emu: &mut Emu) {
    initialize(emu, "OleInitialize", false, true);
}
pub fn CoInitialize(emu: &mut Emu) {
    initialize(emu, "CoInitialize", false, false);
}
pub fn CoInitializeEx(emu: &mut Emu) {
    initialize(emu, "CoInitializeEx", true, false);
}
pub fn CoUninitialize(emu: &mut Emu) {
    uninitialize(emu, "CoUninitialize", false);
}
pub fn OleUninitialize(emu: &mut Emu) {
    uninitialize(emu, "OleUninitialize", true);
}
pub fn OleBuildVersion(emu: &mut Emu) {
    log_red!(emu, "ole32!OleBuildVersion");
    let call = WinApi(Stdcall);
    call.set_return(emu, (OLE_MAJOR_VERSION << 16) | OLE_BUILD);
}
pub fn CoTestCancel(emu: &mut Emu) {
    log_red!(emu, "ole32!CoTestCancel");
    let call = WinApi(Stdcall);
    call.set_return(emu, RPC_E_CALL_COMPLETE);
}
