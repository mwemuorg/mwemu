use crate::emu;
use crate::windows::constants::errors::*;

pub fn RemoveDirectoryTransactedA(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_string(str_ptr);
    log_red!(
        emu,
        "kernel32!RemoveDirectoryTransactedA '{}'  0x{:x}",
        s,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn RemoveDirectoryTransactedW(emu: &mut emu::Emu) {
    let lpPathName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpPathName = emu.maps.read_wide_string(lpPathName as u64);

    log_red!(emu, "kernel32!RemoveDirectoryTransactedW {}", lpPathName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
