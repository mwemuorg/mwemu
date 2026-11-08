use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessesInVirtualizationContext(emu: &mut emu::Emu) {
    let pkg_family_ptr = arg(emu, 0) as u64;
    let count_ptr = arg(emu, 1) as u64;
    let processes_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessesInVirtualizationContext pkg:0x{:x} count:0x{:x} procs:0x{:x}",
        pkg_family_ptr,
        count_ptr,
        processes_ptr
    );
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
