use crate::emu;

pub fn WritePrivateProfileStructA(emu: &mut emu::Emu) {
    let _lpszSection = emu.regs().rcx as u32;
    let _lpszKey = emu.regs().rdx as u32;
    let lpStruct = emu.regs().r8 as u32;
    let uSizeStruct = emu.regs().r9 as u32;
    let _szFile = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let lpStruct = emu.maps.read_string(lpStruct as u64);
    let uSizeStruct = emu.maps.read_string(uSizeStruct as u64);

    log_red!(
        emu,
        "kernel32!WritePrivateProfileStructA {} {}",
        lpStruct,
        uSizeStruct
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
