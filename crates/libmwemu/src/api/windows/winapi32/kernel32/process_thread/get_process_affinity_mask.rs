use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetProcessAffinityMask(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);
    let proc_affinity_mask_ptr = WinApiAbi::arg(emu, 1);
    let sys_affinity_mask_ptr = WinApiAbi::arg(emu, 2);

    emu.maps.write_dword(proc_affinity_mask_ptr, 0x1337);
    emu.maps.write_dword(sys_affinity_mask_ptr, 0x1337);

    log_red!(emu, "kernel32!GetProcessAffinityMask");

    emu.regs_mut().rax = 1;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
