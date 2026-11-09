use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptHashData(emu: &mut emu::Emu) {
    let h_hash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
