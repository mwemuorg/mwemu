use std::sync::atomic::{AtomicU64, Ordering};

use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::emu::Emu;
use crate::emu::object_handle::NativeObject;
use crate::maps::mem64::Permission;
use crate::winapi::common::wait::is_notification_event;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::{
    STATUS_ACCESS_VIOLATION, STATUS_INFO_LENGTH_MISMATCH, STATUS_INVALID_HANDLE, STATUS_SUCCESS,
};
use crate::windows::constants::system::{
    ETW_ADD_NOTIFICATION_EVENT, ETW_NT_FLAGS_TRACE_EVENT_NOREG, ETW_REGISTER_GUIDS,
    ETW_SET_PROVIDER_TRAITS,
};

static NEXT_FAKE_SYNC_HANDLE: AtomicU64 = AtomicU64::new(0x1000);

pub(super) fn trace_control(emu: &mut Emu) {
    let call = WinSyscall;
    let [operation, input, input_size, output, output_size, returned] =
        api_try!(emu, call.args(emu));
    let status = match operation as u32 {
        ETW_REGISTER_GUIDS | ETW_SET_PROVIDER_TRAITS => provider_reply(
            emu,
            operation as u32,
            input,
            input_size as u32,
            output,
            output_size as u32,
            returned,
        ),
        ETW_ADD_NOTIFICATION_EVENT if output == 0 && output_size as u32 == 0 => {
            notification_event(emu, input, input_size as u32, returned)
        }
        _ => return policy::unsupported_with_abi(emu, "ntdll", "NtTraceControl", emu.pc(), call),
    };
    log_orange!(
        emu,
        "NtTraceControl operation={operation:#x} => {status:#x}"
    );
    call.set_return(emu, status);
}

fn notification_event(emu: &mut Emu, input: u64, input_size: u32, returned: u64) -> u64 {
    if input_size != size_of::<u32>() as u32 {
        return STATUS_INFO_LENGTH_MISMATCH;
    }
    let Some(event) = emu.maps.read_dword(input) else {
        return STATUS_ACCESS_VIOLATION;
    };
    if !is_notification_event(emu, event as u64) {
        STATUS_INVALID_HANDLE
    } else if returned != 0
        && (!emu.maps.validate_write_range(returned, 4) || !emu.maps.write_dword(returned, 0))
    {
        STATUS_ACCESS_VIOLATION
    } else {
        STATUS_SUCCESS
    }
}

fn provider_reply(
    emu: &mut Emu,
    operation: u32,
    input: u64,
    input_size: u32,
    output: u64,
    output_size: u32,
    returned: u64,
) -> u64 {
    const REGISTRATION_SIZE: usize = 0xa0;
    const REGISTRATION_HANDLE: usize = 0x18;
    const TRAITS_HEADER_SIZE: usize = 0x18;
    const TRAITS_REPLY_SIZE: usize = 0x78;

    let registration = operation == ETW_REGISTER_GUIDS;
    let (header_size, reply_size) = if registration {
        (REGISTRATION_SIZE, REGISTRATION_SIZE)
    } else {
        (TRAITS_HEADER_SIZE, TRAITS_REPLY_SIZE)
    };
    if (input_size as usize) < header_size || (output_size as usize) < reply_size {
        return STATUS_INFO_LENGTH_MISMATCH;
    }
    if !emu
        .maps
        .can_access_range(input, header_size, Permission::READ)
        || !emu.maps.validate_write_range(output, reply_size as u64)
        || (returned != 0 && !emu.maps.validate_write_range(returned, 4))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    // The input and output can share a buffer. An empty enable mask means no subscribers.
    let mut reply = [0; REGISTRATION_SIZE];
    if registration {
        let handle = emu
            .handle_management
            .insert_native_object(NativeObject::EtwRegistration);
        reply[REGISTRATION_HANDLE..REGISTRATION_HANDLE + size_of::<u64>()]
            .copy_from_slice(&handle.to_le_bytes());
    } else if emu
        .maps
        .read_qword(input)
        .and_then(|h| emu.handle_management.native_object(h))
        != Some(NativeObject::EtwRegistration)
    {
        return STATUS_INVALID_HANDLE;
    }
    emu.maps.write_bytes(output, &reply[..reply_size]);
    if returned != 0 {
        emu.maps.write_dword(returned, reply_size as u32);
    }
    STATUS_SUCCESS
}

pub(super) fn trace_event(emu: &mut Emu) {
    let call = WinSyscall;
    let [_trace_handle, flags, size, event] = api_try!(emu, call.args(emu));
    if flags as u32 != ETW_NT_FLAGS_TRACE_EVENT_NOREG {
        return policy::unsupported_with_abi(emu, "ntdll", "NtTraceEvent", emu.pc(), call);
    }
    let status = if size as u32 == 0 {
        STATUS_INFO_LENGTH_MISMATCH
    } else if !emu
        .maps
        .can_access_range(event, size as u32 as usize, Permission::READ)
    {
        STATUS_ACCESS_VIOLATION
    } else {
        STATUS_SUCCESS
    };
    log_orange!(
        emu,
        "NtTraceEvent unregistered event={event:#x} size={} => {status:#x}",
        size as u32
    );
    call.set_return(emu, status);
}

pub fn next_handle() -> u64 {
    NEXT_FAKE_SYNC_HANDLE.fetch_add(4, Ordering::Relaxed)
}
