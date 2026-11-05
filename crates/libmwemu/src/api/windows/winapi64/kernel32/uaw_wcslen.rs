use crate::emu;

pub fn uaw_wcslen(emu: &mut emu::Emu) {
    let wstr = emu.regs().rcx;

    let s = emu.maps.read_wide_string(wstr);
    let len = s.encode_utf16().count();

    log_red!(emu, "kernel32!uaw_wcslen str:0x{:x} len:{}", wstr, len);

    emu.set_last_error(0);
    emu.regs_mut().rax = len as u64;
}
