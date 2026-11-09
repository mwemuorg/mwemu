use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::get_crypto_key_len;
use crate::windows::constants::get_cryptoalgorithm_name;

pub(super) fn CryptDeriveKey(emu: &mut emu::Emu) {
    let _hprov = emu.regs().rcx;
    let algid = emu.regs().rdx as u32;
    let _hbase_data = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let hkey_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
        emu.maps.write_qword(hkey_ptr, handle);
    }

    emu.regs_mut().rax = 1;
}
