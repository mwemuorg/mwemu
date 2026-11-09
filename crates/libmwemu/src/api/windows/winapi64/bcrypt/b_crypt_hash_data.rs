use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::windows::constants;

pub fn BCryptHashData(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    let pb_input = emu.regs().rdx;
    let cb_input = emu.regs().r8 as usize;
    let flags = emu.regs().r9;

    let len = cb_input.min(0x100_0000);
    if pb_input != 0 && len > 0 {
        let data = emu.maps.read_bytes(pb_input, len);
        crypto_state::hash_data(h_hash, &data);
    }

    log_red!(
        emu,
        "bcrypt!BCryptHashData hash=0x{:x} len={}",
        h_hash,
        cb_input
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
