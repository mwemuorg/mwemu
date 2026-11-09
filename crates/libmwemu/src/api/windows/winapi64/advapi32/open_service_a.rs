use crate::emu;
use crate::winapi::helper;

pub fn OpenServiceA(emu: &mut emu::Emu) {
    let hsc = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let access = emu.regs().r8;
    let name = emu.maps.read_string(name_ptr);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!OpenServiceA scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
        hsc,
        name,
        access,
        hsvc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hsvc;
}
