use crate::api::windows::common::crypto_state;
use crate::emu;

pub(super) fn CryptHashData(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let len = data_len.min(0x100_0000);
    if data_ptr != 0 && len > 0 {
        let data = emu.maps.read_bytes(data_ptr, len);
        crypto_state::hash_data(hhash, data);
    }

    log_red!(
        emu,
        "advapi32!CryptHashData hHash=0x{:x} len={}",
        hhash,
        data_len
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
