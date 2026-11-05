use crate::emu;
use crate::windows::structures;

pub fn VirtualQueryEx(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let out_buff = emu.regs().r8;
    let size = emu.regs().r9 as u32;

    log_red!(
        emu,
        "kernel32!VirtualQueryEx 0x{:x} 0x{:x} {}",
        addr,
        out_buff,
        size
    );

    if size < 30 {
        log::trace!("buffer to short: {}", size);
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    } else {
        let mbi = structures::MemoryBasicInformation::guess(addr, &mut emu.maps);
        mbi.save(out_buff, &mut emu.maps);
        emu.regs_mut().rax = 1;
    }
}
