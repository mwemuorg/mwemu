use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::crypto::*;

pub(super) fn CryptDeriveKey(emu: &mut emu::Emu) {
    let hprov = WinApiAbi::arg(emu, 0);
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let _hbase_data = WinApiAbi::arg(emu, 2) as u32;
    let flags = WinApiAbi::arg(emu, 3) as u32;
    let hkey_ptr = WinApiAbi::arg(emu, 4);

    let alg = get_cryptoalgorithm_name(algid);
    let mut alg_len = get_crypto_key_len(algid);

    let handle = helper::handler_create(alg);
    if alg_len == 0 {
        alg_len = ((flags >> 16) & 0xffff) as usize;
    }
    helper::handler_put_bytes(handle, &vec![0x41u8; alg_len]);
    crypto_state::create_key(handle, algid, alg_len as u32);

    log_red!(emu, "advapi32!CryptDeriveKey {} bits:{}", alg, alg_len);

    if hkey_ptr != 0 {
        emu.maps.write_dword(hkey_ptr, handle as u32);
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
