use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptGetHashParam(emu: &mut emu::Emu) {
    let hhash = WinApiAbi::arg(emu, 0);
    let param = WinApiAbi::arg(emu, 1) as u32;
    let data_ptr = WinApiAbi::arg(emu, 2);
    let len_ptr = WinApiAbi::arg(emu, 3);
    let flags = WinApiAbi::arg(emu, 4) as u32;

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

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
