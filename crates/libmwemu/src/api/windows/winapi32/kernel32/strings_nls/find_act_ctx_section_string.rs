use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn FindActCtxSectionStringA(emu: &mut emu::Emu) {
    let _dwFlags = WinApiAbi::arg(emu, 0) as u32;
    let _lpExtensionGuid = WinApiAbi::arg(emu, 1) as u32;
    let _ulSectionId = WinApiAbi::arg(emu, 2) as u32;
    let lpStringToFind = WinApiAbi::arg(emu, 3) as u32;
    let _ReturnedData = WinApiAbi::arg(emu, 4) as u32;

    let lpStringToFind = emu.maps.read_string(lpStringToFind as u64);

    log_red!(emu, "kernel32!FindActCtxSectionStringA {}", lpStringToFind);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn FindActCtxSectionStringW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!FindActCtxSectionStringW '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 5);
    emu.regs_mut().rax = 1;
}
