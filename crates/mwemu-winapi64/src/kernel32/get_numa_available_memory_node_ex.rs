use crate::emu;

pub fn GetNumaAvailableMemoryNodeEx(emu: &mut emu::Emu) {
    let node = emu.regs().rcx as u8;
    let avail_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaAvailableMemoryNodeEx node:{} ptr:0x{:x}",
        node,
        avail_ptr
    );
    if avail_ptr != 0 {
        emu.maps.write_qword(avail_ptr, 4 * 1024 * 1024 * 1024u64);
    }
    emu.regs_mut().rax = 1;
}
