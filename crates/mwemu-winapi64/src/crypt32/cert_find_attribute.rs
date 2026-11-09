use crate::emu;
use crate::winapi::helper;

pub fn CertFindAttribute(emu: &mut emu::Emu) {
    let store = emu.regs().rcx;
    let find_type = emu.regs().rdx as u32;
    let find_para = emu.regs().r8;
    let prev = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CertFindAttribute store:0x{:x} type:0x{:x}",
        store,
        find_type
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("certfindattribute");
    emu.regs_mut().rax = handle;
}
