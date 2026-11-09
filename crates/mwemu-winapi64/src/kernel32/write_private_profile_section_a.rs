use crate::emu;

pub fn WritePrivateProfileSectionA(emu: &mut emu::Emu) {
    let lpAppName = emu.regs().rcx as u32;
    let lpString = emu.regs().rdx as u32;
    let lpFileName = emu.regs().r8 as u32;

    let lpAppName = emu.maps.read_string(lpAppName as u64);
    let lpString = emu.maps.read_string(lpString as u64);
    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!WritePrivateProfileSectionA {} {} {}",
        lpAppName,
        lpString,
        lpFileName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
