use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn CreateSymbolicLinkTransactedA(emu: &mut emu::Emu) {
    create_symbolic_link_transacted(emu, "CreateSymbolicLinkTransactedA", false);
}

fn create_symbolic_link_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpSymlinkFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpTargetFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpSymlinkFileName = strings::read_text(emu, lpSymlinkFileName as u64, wide);
    let lpTargetFileName = strings::read_text(emu, lpTargetFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn CreateSymbolicLinkTransactedW(emu: &mut emu::Emu) {
    create_symbolic_link_transacted(emu, "CreateSymbolicLinkTransactedW", true);
}
