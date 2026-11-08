use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn OpenEventA(emu: &mut emu::Emu) {
    let _desired_access = arg(emu, 0) as u64;
    let _inherit = arg(emu, 1) as u64;
    let name_ptr = arg(emu, 2) as u64;

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "kernel32!OpenEventA name: '{}'", name);

    let uri = format!("event://{}", name);
    if let Some(h) = helper::handler_find_by_uri(&uri) {
        emu.regs_mut().rax = h;
    } else {
        emu.regs_mut().rax = helper::handler_create(&uri);
    }
    stdcall_ret(emu, 3);
}
