use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::windows::constants;

pub fn BCryptFinishHash(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    let pb_output = emu.regs().rdx;
    let cb_output = emu.regs().r8 as usize;
    let flags = emu.regs().r9;

    if let Some(digest) = crypto_state::get_hash_param(h_hash, crypto_state::HP_HASHVAL) {
        if pb_output != 0 {
            let write_len = digest.len().min(cb_output);
            emu.maps.write_bytes(pb_output, &digest[..write_len]);
        }
        log_red!(
            emu,
            "bcrypt!BCryptFinishHash hash=0x{:x} digest_len={}",
            h_hash,
            digest.len()
        );
    } else {
        log_red!(
            emu,
            "bcrypt!BCryptFinishHash hash=0x{:x} (no state)",
            h_hash
        );
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
