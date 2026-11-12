use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let wc_str = emu.regs().r8;
    let wc_len = emu.regs().r9 as i32;
    let mb_str = WinApiAbi::arg(emu, 4);
    let mb_len = WinApiAbi::arg(emu, 5) as i32;
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
