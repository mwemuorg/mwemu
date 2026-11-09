use crate::emu;

pub fn HeapSetInformation(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let hinfocls = emu.regs().rdx as u32;
    let hinfo = emu.regs().r8;
    let hinfo_sz = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!HeapSetInformation hndl=0x{:x} class={} info=0x{:x} sz={}",
        hndl,
        hinfocls,
        hinfo,
        hinfo_sz
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
