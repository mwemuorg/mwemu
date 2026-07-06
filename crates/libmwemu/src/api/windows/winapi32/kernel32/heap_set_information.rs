use crate::emu;

pub fn HeapSetInformation(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!HeapSetInformation error reading param");
    let hinfocls = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!HeapSetInformation error reading param");
    let hinfo = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernel32!HeapSetInformation error reading param");
    let hinfo_sz = emu
        .maps
        .read_dword(emu.regs().rsp + 12)
        .expect("kernel32!HeapSetInformation error reading param");

    log_red!(emu, "kernel32!HeapSetInformation");

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
