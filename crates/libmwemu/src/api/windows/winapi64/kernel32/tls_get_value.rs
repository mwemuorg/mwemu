use crate::winapi::winapi64::kernel32::LAST_ERROR;
use crate::{emu, windows::constants};

pub fn TlsGetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx as usize; // Parameter passed in RCX in x64

    let val = if idx < emu.tls64().len() {
        // Set last error to SUCCESS when the function succeeds
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_SUCCESS;
        emu.tls64()[idx]
    } else {
        // Set last error to ERROR_INVALID_PARAMETER for invalid index
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INVALID_PARAMETER;
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
