use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::crypto::*;

pub(super) fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hkey = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let uri = format!("crypthash://{}", get_cryptoalgorithm_name(algid));
    let hndl = helper::handler_create(&uri);
    assert!(hndl < 0x00000001_00000000);

    crypto_state::create_hash(hndl, algid);
    emu.maps.write_dword(hash_ptr, hndl as u32);

    log_red!(
        emu,
        "advapi32!CryptCreateHash {} =0x{:x}",
        get_cryptoalgorithm_name(algid),
        hndl
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
