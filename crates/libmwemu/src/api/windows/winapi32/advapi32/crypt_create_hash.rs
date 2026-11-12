use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::crypto::*;

pub(super) fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = WinApiAbi::arg(emu, 0) as u32;
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let hkey = WinApiAbi::arg(emu, 2) as u32;
    let flags = WinApiAbi::arg(emu, 3) as u32;
    let hash_ptr = WinApiAbi::arg(emu, 4);

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
