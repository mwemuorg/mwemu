use crate::emu;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let info_type = emu.regs().rdx;
    let src_ptr = emu.regs().r8;
    let src_len = emu.regs().r9;
    let src = if src_ptr != 0 {
        emu.maps.read_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!GetStringTypeA type:0x{:x} '{}'",
        info_type,
        src
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
