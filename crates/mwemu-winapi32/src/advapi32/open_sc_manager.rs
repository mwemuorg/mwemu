use crate::emu;
use crate::winapi::helper;

pub fn OpenSCManagerA(emu: &mut emu::Emu) {
    let machine_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let database_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let machine = if machine_ptr != 0 {
        emu.maps.read_string(machine_ptr)
    } else {
        String::new()
    };
    let hsc = helper::handler_create("scmanager");
    log_red!(
        emu,
        "advapi32!OpenSCManagerA machine:'{}' access:0x{:x} => 0x{:x}",
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
    let machine_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let database_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
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
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = hsc;
}
