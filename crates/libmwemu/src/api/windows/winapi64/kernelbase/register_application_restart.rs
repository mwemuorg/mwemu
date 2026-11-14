use crate::emu;

pub fn RegisterApplicationRestart(emu: &mut emu::Emu) {
    let cmdline_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let cmdline = if cmdline_ptr != 0 && cmdline_ptr > 0xffff {
        emu.maps.read_wide_string(cmdline_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegisterApplicationRestart cmdline:'{}' flags:0x{:x}",
        cmdline,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
