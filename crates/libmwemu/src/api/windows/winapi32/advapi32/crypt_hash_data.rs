use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptHashData(emu: &mut emu::Emu) {
    let hhash = WinApiAbi::arg(emu, 0);
    let data_ptr = WinApiAbi::arg(emu, 1);
    let data_len = WinApiAbi::arg(emu, 2) as usize;
    let flags = WinApiAbi::arg(emu, 3) as u32;

    let len = data_len.min(0x100_0000);
    if data_ptr != 0 && len > 0 {
        let data = emu.maps.read_bytes(data_ptr, len);
        crypto_state::hash_data(hhash, &data);
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
