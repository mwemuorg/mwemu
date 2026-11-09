use crate::emu;

pub fn GetNumaProcessorNode(emu: &mut emu::Emu) {
    let processor = emu.regs().rcx as u8;
    let node_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaProcessorNode proc:{} ptr:0x{:x}",
        processor,
        node_ptr
    );
    if node_ptr != 0 {
        emu.maps.write_byte(node_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
