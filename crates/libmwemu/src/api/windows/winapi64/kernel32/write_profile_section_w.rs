use crate::emu;

pub fn WriteProfileSectionW(emu: &mut emu::Emu) {
    let lpAppName = emu.regs().rcx as u32;
    let lpString = emu.regs().rdx as u32;

    let lpAppName = emu.maps.read_wide_string(lpAppName as u64);
    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(
        emu,
        "kernel32!WriteProfileSectionW {} {}",
        lpAppName,
        lpString
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
