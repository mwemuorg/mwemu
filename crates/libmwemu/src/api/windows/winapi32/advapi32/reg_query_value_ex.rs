use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    let value_ptr = arg(emu, 1) as u64;
    let type_ptr = arg(emu, 3) as u64;
    let data_ptr = arg(emu, 4) as u64;
    let data_size_ptr = arg(emu, 5) as u64;
    let value = if value_ptr != 0 {
        emu.maps.read_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExA `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    // The registry is not modelled: hand back a well-formed empty value.
    if data_ptr != 0 {
        emu.maps.write_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 1);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 6);
}

pub(super) fn RegQueryValueExW(emu: &mut emu::Emu) {
    let value_ptr = arg(emu, 1) as u64;
    let type_ptr = arg(emu, 3) as u64;
    let data_ptr = arg(emu, 4) as u64;
    let data_size_ptr = arg(emu, 5) as u64;
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExW `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    // The registry is not modelled: hand back a well-formed empty value.
    if data_ptr != 0 {
        emu.maps.write_wide_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 2);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 6);
}
