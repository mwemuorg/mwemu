use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub(super) fn CryptImportKey(emu: &mut emu::Emu) {
    let _hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hpubkey = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    let algid = if data_len >= 12 {
        emu.maps.read_dword(data_ptr + 8).unwrap_or(0x6801)
    } else {
        0x6801
    };

    log_red!(
        emu,
        "advapi32!CryptImportKey data:0x{:x} len:{} alg:0x{:x}",
        data_ptr,
        data_len,
        algid
    );

    if hkey_ptr != 0 {
        let key_handle = helper::handler_create("crypt_key");
        crypto_state::create_key(key_handle, algid, data_len * 8);
        emu.maps.write_dword(hkey_ptr, key_handle as u32);
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
