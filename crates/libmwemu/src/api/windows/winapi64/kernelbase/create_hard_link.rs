use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateHardLinkA(emu: &mut emu::Emu) {
    create_hard_link(emu, "CreateHardLinkA", false);
}

fn create_hard_link(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let name = strings::read_text(emu, arg1, wide);
    let hndl = helper::handler_create(&format!("kb_{}", name));
    log_red!(emu, "kernelbase!{api} '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn CreateHardLinkW(emu: &mut emu::Emu) {
    create_hard_link(emu, "CreateHardLinkW", true);
}
