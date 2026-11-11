//! Minimal IP Helper queries for an environment without configured interfaces.
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::{memory::Call, trace};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

const ULONG_SIZE: usize = size_of::<u32>();
const HOST_NAME_CAPACITY: usize = 132;

struct FixedInfoLayout {
    size: usize,
    node_type_offset: usize,
}

const FIXED_INFO32: FixedInfoLayout = FixedInfoLayout {
    size: 584,
    node_type_offset: 308,
};
const FIXED_INFO64: FixedInfoLayout = FixedInfoLayout {
    size: 600,
    node_type_offset: 320,
};

fn buffer_size(emu: &Emu, address: u64) -> Result<u32, u64> {
    if address == 0
        || !emu
            .maps
            .can_access_range(address, ULONG_SIZE, Permission::READ_WRITE)
    {
        return Err(ERROR_INVALID_PARAMETER);
    }
    emu.maps.read_dword(address).ok_or(ERROR_INVALID_PARAMETER)
}

fn write_sized_buffer(
    emu: &mut Emu,
    buffer: u64,
    size_pointer: u64,
    data: &[u8],
    short_buffer_error: u64,
) -> u64 {
    let capacity = match buffer_size(emu, size_pointer) {
        Ok(size) => size as usize,
        Err(error) => return error,
    };
    if buffer == 0 || capacity < data.len() {
        return if emu.maps.write_dword(size_pointer, data.len() as u32) {
            short_buffer_error
        } else {
            ERROR_INVALID_PARAMETER
        };
    }
    if !emu.maps.validate_write_range(buffer, data.len() as u64)
        || !emu.maps.write_bytes(buffer, data)
    {
        return ERROR_INVALID_PARAMETER;
    }
    ERROR_SUCCESS
}

fn empty_table(emu: &mut Emu) -> u64 {
    let buffer = Call::Api.arg(emu, 0);
    let size_pointer = Call::Api.arg(emu, 1);
    // All four MIB tables begin with a DWORD entry count; there are no rows.
    write_sized_buffer(
        emu,
        buffer,
        size_pointer,
        &0u32.to_le_bytes(),
        ERROR_INSUFFICIENT_BUFFER,
    )
}

fn adapters_info(emu: &mut Emu) -> u64 {
    match buffer_size(emu, Call::Api.arg(emu, 1)) {
        Ok(_) => ERROR_NO_DATA,
        Err(error) => error,
    }
}

fn adapters_addresses(emu: &mut Emu) -> u64 {
    let family = Call::Api.arg(emu, 0) as u32;
    let reserved = Call::Api.arg(emu, 2);
    if !matches!(family, AF_UNSPEC | AF_INET | AF_INET6) || reserved != 0 {
        return ERROR_INVALID_PARAMETER;
    }
    match buffer_size(emu, Call::Api.arg(emu, 4)) {
        Ok(_) => ERROR_NO_DATA,
        Err(error) => error,
    }
}

fn network_parameters(emu: &mut Emu) -> u64 {
    let buffer = Call::Api.arg(emu, 0);
    let size_pointer = Call::Api.arg(emu, 1);
    let layout = if emu.cfg.is_x64() {
        &FIXED_INFO64
    } else {
        &FIXED_INFO32
    };
    // FIXED_INFO contains two pointers, so its layout differs between x86 and x64.
    // Domain, DNS servers, scope and routing flags remain empty in this profile.
    let mut info = vec![0; layout.size];
    let host_name = emu.cfg.host_name.as_bytes();
    let length = host_name.len().min(HOST_NAME_CAPACITY - 1);
    info[..length].copy_from_slice(&host_name[..length]);
    info[layout.node_type_offset..layout.node_type_offset + ULONG_SIZE]
        .copy_from_slice(&BROADCAST_NODETYPE.to_le_bytes());
    write_sized_buffer(emu, buffer, size_pointer, &info, ERROR_BUFFER_OVERFLOW)
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (parameters, arguments, handler): (&str, usize, fn(&mut Emu) -> u64) = match api {
        "GetIfTable" | "GetIpAddrTable" | "GetTcpTable" | "GetUdpTable" => {
            ("table size order", 3, empty_table)
        }
        "GetAdaptersInfo" => ("adapters size", 2, adapters_info),
        "GetAdaptersAddresses" => ("family flags reserved adapters size", 5, adapters_addresses),
        "GetNetworkParams" => ("parameters size", 2, network_parameters),
        _ => return false,
    };
    trace::api(emu, &format!("iphlpapi!{api}"), parameters, true, |emu| {
        let result = handler(emu);
        Call::Api.finish(emu, result, arguments);
    });
    true
}
