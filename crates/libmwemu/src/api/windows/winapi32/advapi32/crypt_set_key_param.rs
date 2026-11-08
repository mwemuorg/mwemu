use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptSetKeyParam(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!CryptSetKeyParam key:0x{:x} param:0x{:x} data:0x{:x}",
        hkey,
        param,
        data_ptr
    );
    let mut data = Vec::new();
    if data_ptr != 0 && emu.maps.is_mapped(data_ptr) {
        for i in 0..16u64 {
            if let Some(b) = emu.maps.read_byte(data_ptr + i) {
                data.push(b);
            } else {
                break;
            }
        }
    }
    crypto_state::set_key_param(hkey, param, &data);
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
