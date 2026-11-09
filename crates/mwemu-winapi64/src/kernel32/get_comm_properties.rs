use crate::emu;

pub fn GetCommProperties(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let prop_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetCommProperties h:0x{:x} ptr:0x{:x}",
        h_file,
        prop_ptr
    );
    if prop_ptr != 0 {
        emu.maps.write_word(prop_ptr, 44);
        emu.maps.write_word(prop_ptr + 2, 1);
    }
    emu.regs_mut().rax = 1;
}
