use crate::emu;
use crate::winapi::helper;

pub fn CreateEventExA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "kernelbase!CreateEventExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    let hndl = helper::handler_create("event://CreateEventExA");
    emu.regs_mut().rax = hndl;
}

pub fn CreateEventExW(emu: &mut emu::Emu) {
    let security = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let access = emu.regs().r9;
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("event_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateEventExW '{}' flags:0x{:x} => 0x{:x}",
        name,
        flags,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
