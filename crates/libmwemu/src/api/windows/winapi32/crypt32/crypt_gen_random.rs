use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let h_prov = WinApiAbi::arg(emu, 0);
    let len = WinApiAbi::arg(emu, 1) as u32;
    let buffer = WinApiAbi::arg(emu, 2);
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptGenRandom prov: 0x{:x} len: {} buf: 0x{:x}",
        h_prov,
        len,
        buffer
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if buffer != 0 {
        for i in 0..len {
            let byte = ((emu.pos as u32)
                .wrapping_mul(1103515245)
                .wrapping_add(i * 12345)) as u8;
            emu.maps.write_byte(buffer + i as u64, byte);
        }
    }
    emu.regs_mut().rax = 1; // TRUE
}
