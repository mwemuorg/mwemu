use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateSymbolicLinkTransactedA(emu: &mut emu::Emu) {
    create_symbolic_link_transacted(emu, "CreateSymbolicLinkTransactedA", false);
}

fn create_symbolic_link_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpSymlinkFileName = WinApiAbi::arg(emu, 0) as u32;
    let lpTargetFileName = WinApiAbi::arg(emu, 1) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 2) as u32;
    let _hTransaction = WinApiAbi::arg(emu, 3) as u32;

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
