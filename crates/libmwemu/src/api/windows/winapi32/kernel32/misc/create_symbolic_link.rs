use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn CreateSymbolicLinkA(emu: &mut emu::Emu) {
    let lpSymlinkFileName = WinApiAbi::arg(emu, 0) as u32;
    let lpTargetFileName = WinApiAbi::arg(emu, 1) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 2) as u32;

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
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!CreateSymbolicLinkW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
