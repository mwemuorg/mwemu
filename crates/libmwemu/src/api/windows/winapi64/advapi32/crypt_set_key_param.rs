use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptSetKeyParam(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let param = emu.regs().rdx as u32;
    let data_ptr = emu.regs().r8;
    let _flags = emu.regs().r9;
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
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
