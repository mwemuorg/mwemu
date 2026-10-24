use crate::emu;
use crate::windows::constants;

pub fn CreateSymbolicLinkTransactedW(emu: &mut emu::Emu) {
    let lpSymlinkFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpTargetFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpSymlinkFileName = emu.maps.read_wide_string(lpSymlinkFileName as u64);
    let lpTargetFileName = emu.maps.read_wide_string(lpTargetFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateSymbolicLinkTransactedW {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
