use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn MultiByteToWideChar(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mb_str = emu.regs().r8;
    let mb_len = emu.regs().r9 as i32;
    let wc_str = WinApiAbi::arg(emu, 4);
    let wc_len = WinApiAbi::arg(emu, 5) as i32;
    let s = emu.maps.read_string(mb_str);
    let slen = s.len();
    if wc_str != 0 && wc_len > 0 {
        emu.maps.write_wide_string(wc_str, &s);
    }
    log_red!(
        emu,
        "kernelbase!MultiByteToWideChar cp:{} '{}' => {}",
        code_page,
        s,
        slen + 1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (slen + 1) as u64;
}
