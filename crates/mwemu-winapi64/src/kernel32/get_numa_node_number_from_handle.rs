use crate::emu;

pub fn GetNumaNodeNumberFromHandle(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let node_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaNodeNumberFromHandle h:0x{:x} ptr:0x{:x}",
        h_file,
        node_ptr
    );
    if node_ptr != 0 {
        emu.maps.write_word(node_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
