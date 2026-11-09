use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::crypto::*;

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let _hprov = emu.regs().rcx;
    let alg_id = emu.regs().rdx as u32;
    let flags = emu.regs().r8;
    let ph_key = emu.regs().r9;

    let alg_name = get_cryptoalgorithm_name(alg_id);
    let uri = format!("cryptkey://{}", alg_name);
    let hndl = helper::handler_create(&uri);

    let key_bits = (flags >> 16) as u32;
    crypto_state::create_key(hndl, alg_id, key_bits);

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(emu, "advapi32!CryptGenKey alg={} =0x{:x}", alg_name, hndl);

    emu.regs_mut().rax = 1;
}
