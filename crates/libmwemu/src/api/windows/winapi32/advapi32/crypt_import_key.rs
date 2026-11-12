use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptImportKey(emu: &mut emu::Emu) {
    let _hprov = WinApiAbi::arg(emu, 0);
    let data_ptr = WinApiAbi::arg(emu, 1);
    let data_len = WinApiAbi::arg(emu, 2) as u32;
    let _hpubkey = WinApiAbi::arg(emu, 3);
    let _flags = WinApiAbi::arg(emu, 4) as u32;
    let hkey_ptr = WinApiAbi::arg(emu, 5);

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
