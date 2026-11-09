use crate::emu;

pub fn GetProcessHandleCount(emu: &mut emu::Emu) {
    let _h_process = emu.regs().rcx;
    let count_ptr = emu.regs().rdx;

    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 32); // mock handle count
    }

    log_red!(emu, "kernel32!GetProcessHandleCount => 32");
    emu.regs_mut().rax = 1;
}
