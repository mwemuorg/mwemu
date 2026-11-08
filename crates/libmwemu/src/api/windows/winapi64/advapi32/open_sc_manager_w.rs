use crate::emu;
use crate::winapi::helper;

pub fn OpenSCManagerW(emu: &mut emu::Emu) {
    let machine_ptr = emu.regs().rcx;
    let database_ptr = emu.regs().rdx;
    let access = emu.regs().r8;
    let machine = if machine_ptr != 0 {
        emu.maps.read_wide_string(machine_ptr)
    } else {
        String::new()
    };
    let hsc = helper::handler_create("scmanager");
    log_red!(
        emu,
        "advapi32!OpenSCManagerW machine:'{}' access:0x{:x} => 0x{:x}",
        machine,
        access,
        hsc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hsc;
}
