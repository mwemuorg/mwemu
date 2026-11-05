use crate::emu;

pub fn GetProcessHeaps(emu: &mut emu::Emu) {
    let num_heaps = emu.regs().rcx as u32;
    let heaps_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetProcessHeaps count:{} ptr:0x{:x}",
        num_heaps,
        heaps_ptr
    );
    if heaps_ptr != 0 && num_heaps >= 1 {
        emu.maps.write_qword(heaps_ptr, 0x00050000);
    }
    emu.regs_mut().rax = 1;
}
