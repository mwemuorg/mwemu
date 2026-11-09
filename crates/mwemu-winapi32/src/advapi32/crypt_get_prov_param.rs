use crate::emu;

pub fn CryptGetProvParam(emu: &mut emu::Emu) {
    let _hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
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
