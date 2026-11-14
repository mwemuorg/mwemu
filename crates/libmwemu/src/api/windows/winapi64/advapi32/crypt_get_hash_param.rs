use crate::api::windows::common::crypto_state;
use crate::emu;

pub(super) fn CryptGetHashParam(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let param = emu.regs().rdx as u32;
    let data_ptr = emu.regs().r8;
    let len_ptr = emu.regs().r9;

    log_red!(
        emu,
        "advapi32!CryptGetHashParam hash:0x{:x} param:0x{:x}",
        hhash,
        param
    );

    if let Some(result) = crypto_state::get_hash_param(hhash, param) {
        if data_ptr != 0 {
            let buf_sz = if len_ptr != 0 {
                emu.maps.read_dword(len_ptr).unwrap_or(0) as usize
            } else {
                result.len()
            };
            let write_len = result.len().min(buf_sz);
            emu.maps.write_bytes(data_ptr, &result[..write_len]);
        }
        if len_ptr != 0 {
            emu.maps.write_dword(len_ptr, result.len() as u32);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
