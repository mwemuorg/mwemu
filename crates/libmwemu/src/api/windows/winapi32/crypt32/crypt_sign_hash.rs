use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSignHash(emu: &mut emu::Emu) {
    let h_hash = WinApiAbi::arg(emu, 0);
    let key_spec = WinApiAbi::arg(emu, 1) as u32;
    let _desc_ptr = WinApiAbi::arg(emu, 2) as u32;
    let _flags = WinApiAbi::arg(emu, 3) as u32;
    let sig_ptr = WinApiAbi::arg(emu, 4);
    let sig_len_ptr = WinApiAbi::arg(emu, 5);
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptSignHash hash: 0x{:x} spec: {}",
        h_hash,
        key_spec
    );
    if sig_ptr == 0 {
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, 128);
        }
    } else {
        for i in 0..128u64 {
            emu.maps.write_byte(sig_ptr + i, 0xAA);
        }
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, 128);
        }
    }
    emu.regs_mut().rax = 1; // TRUE
}
