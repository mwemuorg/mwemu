use crate::emu;
use crate::winapi::helper;

pub fn GetLogicalDrives(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!GetLogicalDrives => 0x4");
    let _h = helper::handler_create("logical_drives");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x4;
}
