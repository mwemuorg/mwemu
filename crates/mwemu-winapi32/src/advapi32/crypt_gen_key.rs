use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::crypto::*;

pub(super) fn CryptGenKey(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

    let uri = format!("cryptkey://{}", get_cryptoalgorithm_name(algid));
    let hndl = helper::handler_create(&uri);
    assert!(hndl < 0x00000001_00000000);

    let key_bits = flags >> 16;
    crypto_state::create_key(hndl, algid, key_bits);

    if hkey_ptr != 0 {
        emu.maps.write_dword(hkey_ptr, hndl as u32);
    }

    log_red!(
        emu,
        "advapi32!CryptGenKey {} =0x{:x}",
        get_cryptoalgorithm_name(algid),
        hndl
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
