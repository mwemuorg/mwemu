use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGetKeyParam(emu: &mut emu::Emu) {
    let h_key = WinApiAbi::arg(emu, 0);
    let param = WinApiAbi::arg(emu, 1) as u32;
    let data_ptr = WinApiAbi::arg(emu, 2);
    let data_len_ptr = WinApiAbi::arg(emu, 3);
    let _flags = WinApiAbi::arg(emu, 4) as u32;
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
