use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptHashData(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let data_ptr = emu.regs().rdx;
    let data_len = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;

    let len = (data_len as usize).min(0x100_0000);
    if data_ptr != 0 && len > 0 {
        let data = emu.maps.read_bytes(data_ptr, len);
        crypto_state::hash_data(hhash, data);
    }

    log_red!(
        emu,
        "advapi32!CryptHashData hHash=0x{:x} data=0x{:x} len={}",
        hhash,
        data_ptr,
        data_len
    );

    emu.regs_mut().rax = 1;
}
