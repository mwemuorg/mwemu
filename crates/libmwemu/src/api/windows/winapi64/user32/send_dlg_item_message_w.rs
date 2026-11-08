use crate::emu;

pub fn SendDlgItemMessageW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "user32!SendDlgItemMessageW '{}'  0x{:x}", s, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
