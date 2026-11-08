use crate::emu;

pub fn HeapSetInformation(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let hinfocls = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hinfo = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let hinfo_sz = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!HeapSetInformation hndl=0x{:x} class={} info=0x{:x} sz={}",
        hndl,
        hinfocls,
        hinfo,
        hinfo_sz
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
