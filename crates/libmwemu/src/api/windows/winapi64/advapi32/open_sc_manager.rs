use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn OpenSCManagerA(emu: &mut emu::Emu) {
    open_sc_manager(emu, "OpenSCManagerA", false);
}

fn open_sc_manager(emu: &mut emu::Emu, api: &str, wide: bool) {
    let machine_ptr = emu.regs().rcx;
    let database_ptr = emu.regs().rdx;
    let access = emu.regs().r8;
    let machine = if machine_ptr != 0 {
        strings::read_text(emu, machine_ptr, wide)
    } else {
        String::new()
    };
    let hsc = helper::handler_create("scmanager");
    log_red!(
        emu,
        "advapi32!{api} machine:'{}' access:0x{:x} => 0x{:x}",
        machine,
        access,
        hsc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hsc;
}

pub fn OpenSCManagerW(emu: &mut emu::Emu) {
    open_sc_manager(emu, "OpenSCManagerW", true);
}
