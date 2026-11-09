use crate::emu;

pub fn GetNumaProcessorNodeEx(emu: &mut emu::Emu) {
    let proc_num_ptr = emu.regs().rcx;
    let node_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaProcessorNodeEx proc:0x{:x} node:0x{:x}",
        proc_num_ptr,
        node_ptr
    );
    if node_ptr != 0 {
        emu.maps.write_word(node_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
