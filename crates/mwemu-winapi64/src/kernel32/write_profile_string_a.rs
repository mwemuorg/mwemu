use crate::emu;

pub fn WriteProfileStringA(emu: &mut emu::Emu) {
    let lpAppName = emu.regs().rcx as u32;
    let lpKeyName = emu.regs().rdx as u32;
    let lpString = emu.regs().r8 as u32;

    let lpAppName = emu.maps.read_string(lpAppName as u64);
    let lpKeyName = emu.maps.read_string(lpKeyName as u64);
    let lpString = emu.maps.read_string(lpString as u64);

    log_red!(
        emu,
        "kernel32!WriteProfileStringA {} {} {}",
        lpAppName,
        lpKeyName,
        lpString
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
