use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::crypto::*;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let _hprov = emu.regs().rcx;
    let alg_id = emu.regs().rdx as u32;
    let _hkey = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    let ph_hash = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let alg_name = get_cryptoalgorithm_name(alg_id);
    let uri = format!("crypthash://{}", alg_name);
    let hndl = helper::handler_create(&uri);

    crypto_state::create_hash(hndl, alg_id);

    if ph_hash != 0 {
        emu.maps.write_qword(ph_hash, hndl);
    }

    log_red!(
        emu,
        "advapi32!CryptCreateHash alg={} =0x{:x}",
        alg_name,
        hndl
    );

    emu.regs_mut().rax = 1;
}
