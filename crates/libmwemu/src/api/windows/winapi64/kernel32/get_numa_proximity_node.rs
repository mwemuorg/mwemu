use crate::emu;

pub fn GetNumaProximityNode(emu: &mut emu::Emu) {
    let proximity_id = emu.regs().rcx as u32;
    let node_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaProximityNode id:{} ptr:0x{:x}",
        proximity_id,
        node_ptr
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if node_ptr != 0 {
        emu.maps.write_byte(node_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
