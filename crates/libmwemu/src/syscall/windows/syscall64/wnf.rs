use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::memory::read_array;
use crate::api::windows::policy;
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::{
    FEATURE_CONFIGURATION_CHANGE_STAMP, WIN11_ETW_WNF_EVENT_MASK, WIN11_ETW_WNF_STATE,
    WNF_CMFC_FEATURE_CONFIGURATION_CHANGED,
};

const INITIAL_WNF_CHANGE_STAMP: u32 = 1;
const WNF_WIL_FEATURE_STORE: u64 = 0x418a_073a_a3bc_0c75;
const WNF_WIL_BOOT_FEATURE_STORE: u64 = 0x418a_073a_a3bc_1475;
const WNF_WIL_MACHINE_FEATURE_STORE: u64 = 0x418a_073a_a3bc_7c75;
const WNF_WIL_USER_FEATURE_STORE: u64 = 0x418a_073a_a3bc_88f5;

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct WnfSubscriptions {
    last_id: u64,
    active: BTreeMap<u64, u64>,
}

impl WnfSubscriptions {
    fn subscribe(&mut self, state: u64) -> Result<u64, u64> {
        if let Some(&id) = self.active.get(&state) {
            return Ok(id);
        }
        self.last_id = self.last_id.checked_add(1).ok_or(STATUS_NO_MEMORY)?;
        self.active.insert(state, self.last_id);
        Ok(self.last_id)
    }
}

pub(super) fn subscribe(emu: &mut Emu) {
    let call = WinSyscall;
    let [name, stamp, mask, output] = api_try!(emu, call.args(emu));
    if mask as u32 != WIN11_ETW_WNF_EVENT_MASK {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtSubscribeWnfStateChange",
            emu.pc(),
            call,
        );
    }
    let Some(state) = emu.maps.read_qword(name) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    // These immutable states have no pending change when the caller has the current stamp.
    if !matches!(
        (state, stamp as u32),
        (WIN11_ETW_WNF_STATE, 0)
            | (
                WNF_CMFC_FEATURE_CONFIGURATION_CHANGED,
                INITIAL_WNF_CHANGE_STAMP
            )
    ) {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtSubscribeWnfStateChange",
            emu.pc(),
            call,
        );
    }
    if output != 0 && !emu.maps.validate_write_range(output, 8) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let id = match emu.wnf_subscriptions.subscribe(state) {
        Ok(id) => id,
        Err(status) => return call.set_return(emu, status),
    };
    if output != 0 {
        emu.maps.write_qword(output, id);
    }
    log_orange!(
        emu,
        "NtSubscribeWnfStateChange state={state:#x} mask={mask:#x} id={id:#x}"
    );
    call.set_return(emu, STATUS_SUCCESS);
}

pub(super) fn unsubscribe(emu: &mut Emu) {
    let call = WinSyscall;
    let name = api_try!(emu, call.arg(emu, 0));
    let Some(state) = emu.maps.read_qword(name) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    if !matches!(
        state,
        WIN11_ETW_WNF_STATE | WNF_CMFC_FEATURE_CONFIGURATION_CHANGED
    ) {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtUnsubscribeWnfStateChange",
            emu.pc(),
            call,
        );
    }
    let status = if emu.wnf_subscriptions.active.remove(&state).is_some() {
        STATUS_SUCCESS
    } else {
        STATUS_OBJECT_NAME_NOT_FOUND
    };
    log_orange!(
        emu,
        "NtUnsubscribeWnfStateChange state={state:#x} => {status:#x}"
    );
    call.set_return(emu, status);
}

pub(super) fn query_state_data(emu: &mut Emu) {
    let call = WinSyscall;
    let [state, type_id, scope, stamp, data, size] = api_try!(emu, call.args(emu));
    let Some(state) = emu.maps.read_qword(state) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    if state == WNF_CMFC_FEATURE_CONFIGURATION_CHANGED && scope == 0 {
        let status = query_feature_change(emu, type_id, stamp, data, size);
        return call.set_return(emu, status);
    }
    if type_id != 0
        || scope != 0
        || !matches!(
            state,
            WNF_WIL_FEATURE_STORE
                | WNF_WIL_BOOT_FEATURE_STORE
                | WNF_WIL_MACHINE_FEATURE_STORE
                | WNF_WIL_USER_FEATURE_STORE
        )
    {
        return policy::unsupported_with_abi(emu, "ntdll", "NtQueryWnfStateData", emu.pc(), call);
    }
    if !emu.maps.validate_write_range(stamp, 4) || !emu.maps.validate_write_range(size, 4) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    emu.maps.write_dword(stamp, 0);
    emu.maps.write_dword(size, 0);
    call.set_return(emu, STATUS_SUCCESS);
}

fn query_feature_change(emu: &mut Emu, type_id: u64, stamp: u64, data: u64, size: u64) -> u64 {
    // 48f96463-52d6-4e85-9c02-f2c14e17264e in guest GUID layout.
    const FEATURE_CHANGE_TYPE: [u8; 16] = [
        0x63, 0x64, 0xf9, 0x48, 0xd6, 0x52, 0x85, 0x4e, 0x9c, 0x02, 0xf2, 0xc1, 0x4e, 0x17, 0x26,
        0x4e,
    ];
    const REQUIRED: u32 = size_of::<u64>() as u32;
    if type_id == 0 {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(type_guid) = read_array(emu, type_id) else {
        return STATUS_ACCESS_VIOLATION;
    };
    if type_guid != FEATURE_CHANGE_TYPE {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(capacity) = emu.maps.read_dword(size) else {
        return STATUS_ACCESS_VIOLATION;
    };
    if !emu.maps.validate_write_range(size, 4)
        || !emu.maps.validate_write_range(stamp, 4)
        || (capacity >= REQUIRED && !emu.maps.validate_write_range(data, REQUIRED as u64))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    // The emulator publishes its initial feature configuration once.
    emu.maps.write_dword(stamp, INITIAL_WNF_CHANGE_STAMP);
    emu.maps.write_dword(size, REQUIRED);
    if capacity < REQUIRED {
        return STATUS_BUFFER_TOO_SMALL;
    }
    emu.maps
        .write_qword(data, FEATURE_CONFIGURATION_CHANGE_STAMP);
    STATUS_SUCCESS
}
