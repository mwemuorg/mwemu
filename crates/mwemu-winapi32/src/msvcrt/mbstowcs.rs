use crate::emu;

pub fn mbstowcs(emu: &mut emu::Emu, api: &str) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let s = emu.maps.read_string(src);
    let copy_len = s.len().min(n);
    for (i, b) in s.as_bytes()[..copy_len].iter().enumerate() {
        emu.maps.write_word(dst + (i * 2) as u64, *b as u16);
    }
    if copy_len < n {
        emu.maps.write_word(dst + (copy_len * 2) as u64, 0);
    }
    log_red!(emu, "msvcrt!{} '{}' => {}", api, s, copy_len);
    emu.regs_mut().rax = copy_len as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
