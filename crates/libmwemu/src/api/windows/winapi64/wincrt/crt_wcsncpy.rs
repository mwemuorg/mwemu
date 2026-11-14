use crate::emu;

pub(super) fn crt_wcsncpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let ws = emu.maps.read_wide_string(src);
    let chars: Vec<u16> = ws.encode_utf16().collect();
    let copy_len = chars.len().min(n as usize);
    for (i, c) in chars.iter().take(copy_len).enumerate() {
        emu.maps.write_word(dst + (i as u64) * 2, *c);
    }
    for i in copy_len..n as usize {
        emu.maps.write_word(dst + (i as u64) * 2, 0);
    }
    emu.regs_mut().rax = dst;
}
