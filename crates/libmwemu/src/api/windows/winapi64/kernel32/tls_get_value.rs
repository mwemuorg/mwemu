use crate::{emu, windows::constants};

pub fn TlsGetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx as usize; // Parameter passed in RCX in x64

    let val = if idx < emu.tls64().len() {
        emu.set_last_error(constants::ERROR_SUCCESS);
        emu.tls64()[idx]
    } else {
        emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
        0
    };

    emu.regs_mut().rax = val;

    log_red!(
        emu,
        "** {} kernel32!TlsGetValue idx: {} =0x{:x}",
        emu.pos,
        idx,
        val
    );
}
