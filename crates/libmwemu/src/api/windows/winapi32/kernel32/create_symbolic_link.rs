use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn CreateSymbolicLinkA(emu: &mut emu::Emu) {
    let lpSymlinkFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpTargetFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpSymlinkFileName = emu.maps.read_string(lpSymlinkFileName as u64);
    let lpTargetFileName = emu.maps.read_string(lpTargetFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateSymbolicLinkA {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn CreateSymbolicLinkW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!CreateSymbolicLinkW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
