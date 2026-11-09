use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptHashData(emu: &mut emu::Emu) {
    let h_hash = arg(emu, 0) as u64;
    let pb_input = arg(emu, 1) as u64;
    let cb_input = arg(emu, 2) as usize;
    let _flags = arg(emu, 3);

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

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
