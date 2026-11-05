use crate::emu;

pub fn GetProcessTimes(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let creation_ptr = emu.regs().rdx;
    let exit_ptr = emu.regs().r8;
    let kernel_ptr = emu.regs().r9;
    let user_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(emu, "kernel32!GetProcessTimes h:0x{:x}", h_process);
    let filetime: u64 = 132514560000000000;
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, filetime);
    }
    if exit_ptr != 0 {
        emu.maps.write_qword(exit_ptr, 0);
    }
    if kernel_ptr != 0 {
        emu.maps.write_qword(kernel_ptr, 100_000);
    }
    if user_ptr != 0 {
        emu.maps.write_qword(user_ptr, 500_000);
    }
    emu.regs_mut().rax = 1;
}
