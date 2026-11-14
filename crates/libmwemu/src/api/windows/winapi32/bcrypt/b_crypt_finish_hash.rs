use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptFinishHash(emu: &mut emu::Emu) {
    let h_hash = arg(emu, 0) as u64;
    let pb_output = arg(emu, 1) as u64;
    let cb_output = arg(emu, 2) as usize;
    let _flags = arg(emu, 3);

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

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
