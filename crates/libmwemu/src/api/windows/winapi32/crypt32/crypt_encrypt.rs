use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptEncrypt(emu: &mut emu::Emu) {
    let h_key = WinApiAbi::arg(emu, 0);
    let h_hash = WinApiAbi::arg(emu, 1);
    let is_final = WinApiAbi::arg(emu, 2) as u32;
    let dw_flags = WinApiAbi::arg(emu, 3) as u32;
    let pb_data = WinApiAbi::arg(emu, 4);
    let pdw_data_len = WinApiAbi::arg(emu, 5);
    let dw_buf_len = WinApiAbi::arg(emu, 6) as u32;
    log_red!(
        emu,
        "crypt32!CryptEncrypt key:0x{:x} hash:0x{:x} final:{} flags:0x{:x} data:0x{:x} buf:{}",
        h_key,
        h_hash,
        is_final,
        dw_flags,
        pb_data,
        dw_buf_len
    );
    if pb_data != 0 && pdw_data_len != 0 {
        let data_len = emu.maps.read_dword(pdw_data_len).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(pb_data, len).to_vec();
            crypto_state::encrypt_data(h_key, &mut data);
            emu.maps.write_bytes(pb_data, &data);
        }
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
}
