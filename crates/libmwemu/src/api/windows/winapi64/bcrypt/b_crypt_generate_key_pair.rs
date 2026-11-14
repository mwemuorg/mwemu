use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn BCryptGenerateKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let ph_key = emu.regs().rdx;
    let length = emu.regs().r8 as u32;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkeypair://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateKeyPair algo={} bits={} =0x{:x}",
        algo_name,
        length,
        hndl
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}
