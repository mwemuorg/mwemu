use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptImportKey(emu: &mut emu::Emu) {
    let h_prov = WinApiAbi::arg(emu, 0);
    let data_ptr = WinApiAbi::arg(emu, 1);
    let data_len = WinApiAbi::arg(emu, 2) as u32;
    let h_pub_key = WinApiAbi::arg(emu, 3);
    let _flags = WinApiAbi::arg(emu, 4) as u32;
    let h_key_ptr = WinApiAbi::arg(emu, 5);
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptImportKey prov: 0x{:x} data: 0x{:x} len: {} pubkey: 0x{:x}",
        h_prov,
        data_ptr,
        data_len,
        h_pub_key
    );
    let h_key = helper::handler_create("importedkey");
    crypto_state::create_key(h_key, 0x6801, data_len * 8);
    if h_key_ptr != 0 {
        emu.maps.write_dword(h_key_ptr, h_key as u32);
    }
    emu.regs_mut().rax = 1; // TRUE
}
