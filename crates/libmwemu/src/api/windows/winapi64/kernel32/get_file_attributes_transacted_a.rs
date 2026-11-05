use crate::emu;
use crate::windows::constants;

pub fn GetFileAttributesTransactedA(emu: &mut emu::Emu) {
    let lpFileName = emu.regs().rcx as u32;
    let _fInfoLevelId = emu.regs().rdx as u32;
    let _lpFileInformation = emu.regs().r8 as u32;
    let _hTransaction = emu.regs().r9 as u32;

    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(emu, "kernel32!GetFileAttributesTransactedA {}", lpFileName);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
