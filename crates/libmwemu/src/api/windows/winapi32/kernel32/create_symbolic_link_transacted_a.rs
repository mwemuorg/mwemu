use crate::emu;
use crate::windows::constants;

pub fn CreateSymbolicLinkTransactedA(emu: &mut emu::Emu) {
    let lpSymlinkFileName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpTargetFileName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _dwFlags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _hTransaction = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");

    let lpSymlinkFileName = emu.maps.read_string(lpSymlinkFileName as u64);
    let lpTargetFileName = emu.maps.read_string(lpTargetFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateSymbolicLinkTransactedA {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
