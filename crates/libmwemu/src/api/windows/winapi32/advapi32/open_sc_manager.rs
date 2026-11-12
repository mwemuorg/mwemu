use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn OpenSCManagerA(emu: &mut emu::Emu) {
    open_sc_manager(emu, "OpenSCManagerA", false);
}

fn open_sc_manager(emu: &mut emu::Emu, api: &str, wide: bool) {
    let machine_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let database_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
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
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = hsc;
}

pub fn OpenSCManagerW(emu: &mut emu::Emu) {
    open_sc_manager(emu, "OpenSCManagerW", true);
}
