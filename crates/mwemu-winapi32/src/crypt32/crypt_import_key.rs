use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptImportKey(emu: &mut emu::Emu) {
    let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let h_pub_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let h_key_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
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
