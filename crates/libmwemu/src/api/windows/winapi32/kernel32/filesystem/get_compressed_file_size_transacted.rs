use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn GetCompressedFileSizeTransactedA(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_string(str_ptr);
    log_red!(
        emu,
        "kernel32!GetCompressedFileSizeTransactedA '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}

pub fn GetCompressedFileSizeTransactedW(emu: &mut emu::Emu) {
    let lpFileName = WinApiAbi::arg(emu, 0) as u32;
    let _lpFileSizeHigh = WinApiAbi::arg(emu, 1) as u32;
    let _hTransaction = WinApiAbi::arg(emu, 2) as u32;

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!GetCompressedFileSizeTransactedW {}",
        lpFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
