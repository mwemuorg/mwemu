use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn HeapSetInformation(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0) as u32;
    let hinfocls = WinApiAbi::arg(emu, 1) as u32;
    let hinfo = WinApiAbi::arg(emu, 2) as u32;
    let hinfo_sz = WinApiAbi::arg(emu, 3) as u32;

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
