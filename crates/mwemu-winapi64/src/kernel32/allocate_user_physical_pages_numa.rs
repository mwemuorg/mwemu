use crate::emu;

pub fn AllocateUserPhysicalPagesNuma(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let p_page_count = emu.regs().rdx;
    let p_pages = emu.regs().r8;
    let numa_node = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!AllocateUserPhysicalPagesNuma hProcess:0x{:x} pPageCount:0x{:x} pPages:0x{:x} node:{}",
        h_process,
        p_page_count,
        p_pages,
        numa_node
    );

    if p_page_count != 0 {
        emu.maps.write_qword(p_page_count, 0);
    }
    emu.regs_mut().rax = 1;
}
