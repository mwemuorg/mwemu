use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn AllocateUserPhysicalPages(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let p_page_count = arg(emu, 1) as u64;
    let p_pages = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!AllocateUserPhysicalPages hProcess:0x{:x} pPageCount:0x{:x} pPages:0x{:x}",
        h_process,
        p_page_count,
        p_pages
    );

    if p_page_count != 0 {
        emu.maps.write_qword(p_page_count, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
