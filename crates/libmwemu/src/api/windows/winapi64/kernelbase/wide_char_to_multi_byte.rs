use crate::emu;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let wc_str = emu.regs().r8;
    let wc_len = emu.regs().r9 as i32;
    let mb_str = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let mb_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let s = emu.maps.read_wide_string(wc_str);
    let slen = s.len();
    if mb_str != 0 && mb_len > 0 {
        emu.maps.write_string(mb_str, &s);
    }
    log_red!(
        emu,
        "kernelbase!WideCharToMultiByte cp:{} '{}' => {}",
        code_page,
        s,
        slen + 1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (slen + 1) as u64;
}
