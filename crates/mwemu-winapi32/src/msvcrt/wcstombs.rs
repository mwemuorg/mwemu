use crate::emu;

pub fn wcstombs(emu: &mut emu::Emu, api: &str) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let ws = emu.maps.read_wide_string(src);
    let copy_len = ws.len().min(n);
    for (i, b) in ws.as_bytes()[..copy_len].iter().enumerate() {
        emu.maps.write_byte(dst + i as u64, *b);
    }
    if copy_len < n {
        emu.maps.write_byte(dst + copy_len as u64, 0);
    }
    log_red!(emu, "msvcrt!{} '{}' => {}", api, ws, copy_len);
    emu.regs_mut().rax = copy_len as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
