use crate::emu;
use crate::winapi::helper;

pub fn CreateDirectory2A(emu: &mut emu::Emu) {
    create_directory2(emu, "CreateDirectory2A");
}

fn create_directory2(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateDirectory2W(emu: &mut emu::Emu) {
    create_directory2(emu, "CreateDirectory2W");
}
