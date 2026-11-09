use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptGetKeyParam(emu: &mut emu::Emu) {
    let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    log_red!(
        emu,
        "crypt32!CryptGetKeyParam key:0x{:x} param:{} data:0x{:x}",
        h_key,
        param,
        data_ptr
    );
    if let Some(result) = crypto_state::get_key_param(h_key, param) {
        if data_ptr != 0 {
            emu.maps.write_bytes(data_ptr, &result);
        }
        if data_len_ptr != 0 {
            emu.maps.write_dword(data_len_ptr, result.len() as u32);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
}
