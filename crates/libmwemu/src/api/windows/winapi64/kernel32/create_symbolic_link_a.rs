use crate::emu;
use crate::winapi::helper;

pub fn CreateSymbolicLinkA(emu: &mut emu::Emu) {
    let lpSymlinkFileName = emu.regs().rcx as u32;
    let lpTargetFileName = emu.regs().rdx as u32;
    let _dwFlags = emu.regs().r8 as u32;

    let lpSymlinkFileName = emu.maps.read_string(lpSymlinkFileName as u64);
    let lpTargetFileName = emu.maps.read_string(lpTargetFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateSymbolicLinkA {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    emu.regs_mut().rax = helper::handler_create("CreateSymbolicLinkA");
}
