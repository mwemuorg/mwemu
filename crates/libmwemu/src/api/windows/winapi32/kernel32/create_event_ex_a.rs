use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateEventExA(emu: &mut emu::Emu) {
    let attr = arg(emu, 0) as u64;
    let name_ptr = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let desired_access = arg(emu, 3) as u64;

    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!CreateEventExA attr: 0x{:x} name: '{}' flags: 0x{:x} access: 0x{:x}",
        attr,
        name,
        flags,
        desired_access
    );

    let uri = format!("event://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
    stdcall_ret(emu, 4);
}
