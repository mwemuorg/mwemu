use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptDecrypt(emu: &mut emu::Emu) {
    let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let h_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let is_final = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pdw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptDecrypt key:0x{:x} hash:0x{:x} final:{} flags:0x{:x} data:0x{:x}",
        h_key,
        h_hash,
        is_final,
        dw_flags,
        pb_data
    );
    if pb_data != 0 && pdw_data_len != 0 {
        let data_len = emu.maps.read_dword(pdw_data_len).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(pb_data, len).to_vec();
            crypto_state::decrypt_data(h_key, &mut data);
            emu.maps.write_bytes(pb_data, &data);
        }
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
}
