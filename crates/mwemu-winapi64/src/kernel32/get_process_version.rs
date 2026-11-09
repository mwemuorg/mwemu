use crate::emu;

pub fn GetProcessVersion(emu: &mut emu::Emu) {
    let process_id = emu.regs().rcx as u32;
    log_red!(
        emu,
        "kernel32!GetProcessVersion pid:0x{:x} => 0x000A0000",
        process_id
    );
    let _ = emu.maps.is_mapped(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x000A0000;
}
