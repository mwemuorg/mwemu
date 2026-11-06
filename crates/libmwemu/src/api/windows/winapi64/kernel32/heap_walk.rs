use crate::emu;

pub fn HeapWalk(emu: &mut emu::Emu) {
    let heap = emu.regs().rcx;
    let entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!HeapWalk heap:0x{:x} entry:0x{:x}",
        heap,
        entry
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // FALSE (no more entries)
}
