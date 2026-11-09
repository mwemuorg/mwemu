use crate::emu;

pub fn CryptGetProvParam(emu: &mut emu::Emu) {
    let _hprov = emu.regs().rcx;
    let param = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
    let data_len_ptr = emu.regs().r9;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
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
    emu.regs_mut().rax = 1;
}
