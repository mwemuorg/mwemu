use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetProcessTimes(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let creation_ptr = emu.regs().rdx;
    let exit_ptr = emu.regs().r8;
    let kernel_ptr = emu.regs().r9;
    let user_ptr = WinApiAbi::arg(emu, 4);
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, 0x01D0_0000_0000_0000);
    }
    if exit_ptr != 0 {
        emu.maps.write_qword(exit_ptr, 0);
    }
    if kernel_ptr != 0 {
        emu.maps.write_qword(kernel_ptr, 1000000);
    }
    if user_ptr != 0 {
        emu.maps.write_qword(user_ptr, 5000000);
    }
    log_red!(emu, "kernelbase!GetProcessTimes h:0x{:x}", handle);
    let _ = emu.maps.is_mapped(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x000A0000;
}
