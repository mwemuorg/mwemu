use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub(super) fn CryptImportKey(emu: &mut emu::Emu) {
    let _hprov = emu.regs().rcx;
    let data_ptr = emu.regs().rdx;
    let data_len = emu.regs().r8 as u32;
    let _hpubkey = emu.regs().r9;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let hkey_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

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
        emu.maps.write_qword(hkey_ptr, key_handle);
    }

    emu.regs_mut().rax = 1;
}
