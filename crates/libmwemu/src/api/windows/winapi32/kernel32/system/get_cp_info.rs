use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetCPInfo(emu: &mut emu::Emu) {
    let codepage = WinApiAbi::arg(emu, 0) as u32;
    let info_ptr = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!GetCPInfo {} 0x{}", codepage, info_ptr);

    // TODO: put something in lp_cp_info?

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;

    // https://learn.microsoft.com/en-us/windows/win32/api/winnls/ns-winnls-cpinfo
}
