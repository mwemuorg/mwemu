use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn CommConfigDialogA(emu: &mut emu::Emu) {
    let lpszName = WinApiAbi::arg(emu, 0) as u32;
    let _hWnd = WinApiAbi::arg(emu, 1) as u32;
    let _lpCC = WinApiAbi::arg(emu, 2) as u32;

    let lpszName = emu.maps.read_string(lpszName as u64);

    log_red!(emu, "kernel32!CommConfigDialogA {}", lpszName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn CommConfigDialogW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!CommConfigDialogW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
