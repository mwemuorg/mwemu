use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn WriteConsoleA(emu: &mut Emu) {
    invoke(emu, false, "WriteConsoleA");
}

pub fn WriteConsoleW(emu: &mut Emu) {
    invoke(emu, true, "WriteConsoleW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let input = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let written = emu.regs().r9;
    let reserved = WinApiAbi::arg(emu, 4);
    let size = count as usize
        * if wide {
            size_of::<u16>()
        } else {
            size_of::<u8>()
        };
    log_red!(
        emu,
        "kernel32!{} handle:0x{:x} input:0x{:x} count:{}",
        api,
        emu.regs().rcx,
        input,
        count
    );
    let error = if reserved != 0 {
        ERROR_INVALID_PARAMETER
    } else if !emu.maps.can_access_range(input, size, Permission::READ)
        || (written != 0 && !emu.maps.write_dword(written, count))
    {
        ERROR_NOACCESS
    } else {
        ERROR_SUCCESS
    };
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    emu.regs_mut().rax = (error == ERROR_SUCCESS) as u64;
}
