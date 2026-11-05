use crate::emu;
use crate::winapi::helper;

pub fn OpenEventA(emu: &mut emu::Emu) {
    let _desired_access = emu.regs().rcx;
    let _inherit = emu.regs().rdx;
    let name_ptr = emu.regs().r8;

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "kernel32!OpenEventA name: '{}'", name);

    let uri = format!("event://{}", name);
    if let Some(h) = helper::handler_find_by_uri(&uri) {
        emu.regs_mut().rax = h;
    } else {
        emu.regs_mut().rax = helper::handler_create(&uri);
    }
}
