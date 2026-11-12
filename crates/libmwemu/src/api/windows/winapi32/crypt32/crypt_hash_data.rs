use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptHashData(emu: &mut emu::Emu) {
    let h_hash = WinApiAbi::arg(emu, 0);
    let pb_data = WinApiAbi::arg(emu, 1);
    let dw_data_len = WinApiAbi::arg(emu, 2) as u32;
    let dw_flags = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "crypt32!CryptHashData hash:0x{:x} data:0x{:x} len:{} flags:0x{:x}",
        h_hash,
        pb_data,
        dw_data_len,
        dw_flags
    );
    if pb_data != 0 && dw_data_len > 0 && dw_data_len < 0x100000 {
        let data = emu.maps.read_bytes(pb_data, dw_data_len as usize).to_vec();
        crypto_state::hash_data(h_hash, &data);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
