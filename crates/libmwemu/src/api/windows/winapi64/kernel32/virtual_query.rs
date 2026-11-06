use crate::emu;
use crate::windows::structures;

pub fn VirtualQuery(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let out_buff = emu.regs().rdx;
    let size = emu.regs().r8 as u32;

    log_red!(
        emu,
        "kernel32!VirtualQuery 0x{:x} 0x{:x} {}",
        addr,
        out_buff,
        size
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

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
