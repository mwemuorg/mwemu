use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strncat(emu: &mut emu::Emu) {
    let dst = WinApiAbi::arg(emu, 0);
    let src = WinApiAbi::arg(emu, 1);
    let n = WinApiAbi::arg(emu, 2) as usize;
    let dst_s = emu.maps.read_string(dst);
    let src_s = emu.maps.read_string(src);
    let copy_len = src_s.len().min(n);
    let offset = dst + dst_s.len() as u64;
    for (i, b) in src_s.as_bytes()[..copy_len].iter().enumerate() {
        emu.maps.write_byte(offset + i as u64, *b);
    }
    emu.maps.write_byte(offset + copy_len as u64, 0);
    log_red!(emu, "msvcrt!strncat 0x{:x} '{}' {}", dst, src_s, n);
    emu.regs_mut().rax = dst;
}
