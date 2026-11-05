use crate::emu;

pub fn GetNumaProximityNodeEx(emu: &mut emu::Emu) {
    let proximity_id = emu.regs().rcx as u32;
    let node_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaProximityNodeEx id:{} ptr:0x{:x}",
        proximity_id,
        node_ptr
    );
    if node_ptr != 0 {
        emu.maps.write_word(node_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
