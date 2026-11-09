use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexExW(emu: &mut emu::Emu) {
    let attr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let desired_access = emu.regs().r9;

    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!CreateMutexExW attr: 0x{:x} name: '{}' flags: 0x{:x} access: 0x{:x}",
        attr,
        name,
        flags,
        desired_access
    );

    let uri = format!("mutex://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
