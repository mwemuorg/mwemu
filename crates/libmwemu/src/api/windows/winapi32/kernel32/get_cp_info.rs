use crate::emu;

pub fn GetCPInfo(emu: &mut emu::Emu) {
    let codepage = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!GetCPInfo error reading codepage param");
    let info_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!GetCPInfo error reading inmfo_ptr param");

    log_red!(emu, "kernel32!GetCPInfo {} 0x{}", codepage, info_ptr);

    // TODO: put something in lp_cp_info?

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;

    // https://learn.microsoft.com/en-us/windows/win32/api/winnls/ns-winnls-cpinfo
}
