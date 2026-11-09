use crate::emu;

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let out_buf = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!GetUserDefaultLCID {}:0x{:x} {}:0x{:x}",
        "handle",
        handle,
        "out_buf",
        out_buf
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
