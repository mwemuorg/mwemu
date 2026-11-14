use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::crypto::*;

pub(super) fn CryptGenKey(emu: &mut emu::Emu) {
    let hprov = WinApiAbi::arg(emu, 0) as u32;
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let flags = WinApiAbi::arg(emu, 2) as u32;
    let hkey_ptr = WinApiAbi::arg(emu, 3);

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
