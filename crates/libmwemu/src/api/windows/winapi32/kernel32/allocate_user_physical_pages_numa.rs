use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn AllocateUserPhysicalPagesNuma(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let p_page_count = arg(emu, 1) as u64;
    let p_pages = arg(emu, 2) as u64;
    let numa_node = arg(emu, 3) as u64;

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
    stdcall_ret(emu, 4);
}
