use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexExW(emu: &mut emu::Emu) {
    let security = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let access = emu.regs().r9;
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("mutex_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateMutexExW '{}' flags:0x{:x} => 0x{:x}",
        name,
        flags,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
