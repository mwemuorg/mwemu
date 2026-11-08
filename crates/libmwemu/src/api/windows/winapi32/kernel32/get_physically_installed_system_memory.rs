use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetPhysicallyInstalledSystemMemory(emu: &mut emu::Emu) {
    let mem_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetPhysicallyInstalledSystemMemory ptr:0x{:x}",
        mem_ptr
    );
    if mem_ptr != 0 {
        emu.maps.write_qword(mem_ptr, 8 * 1024 * 1024);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
