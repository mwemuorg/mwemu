use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGetProvParam(emu: &mut emu::Emu) {
    let _hprov = WinApiAbi::arg(emu, 0);
    let param = WinApiAbi::arg(emu, 1) as u32;
    let data_ptr = WinApiAbi::arg(emu, 2);
    let data_len_ptr = WinApiAbi::arg(emu, 3);
    let _flags = WinApiAbi::arg(emu, 4) as u32;
    log_red!(emu, "advapi32!CryptGetProvParam param:{}", param);
    match param {
        4 => {
            let name = "Microsoft Base Cryptographic Provider v1.0";
            if data_ptr != 0 {
                emu.maps.write_string(data_ptr, name);
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, name.len() as u32 + 1);
            }
        }
        6 => {
            let name = "default";
            if data_ptr != 0 {
                emu.maps.write_string(data_ptr, name);
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, name.len() as u32 + 1);
            }
        }
        _ => {
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, 0);
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 4);
            }
        }
    }
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
