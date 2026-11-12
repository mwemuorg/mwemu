use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn FindActCtxSectionStringA(emu: &mut emu::Emu) {
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpExtensionGuid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _ulSectionId = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpStringToFind = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _ReturnedData = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let lpStringToFind = emu.maps.read_string(lpStringToFind as u64);

    log_red!(emu, "kernel32!FindActCtxSectionStringA {}", lpStringToFind);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn FindActCtxSectionStringW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
