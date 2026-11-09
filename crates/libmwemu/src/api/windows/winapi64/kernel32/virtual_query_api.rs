use crate::emu;
use crate::windows::structures;

pub fn VirtualQuery_api(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let out_buff = emu.regs().rdx;
    let size = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!VirtualQuery 0x{:x} 0x{:x} {}",
        addr,
        out_buff,
        size
    );
    if size < structures::MemoryBasicInformation64::SIZE {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    } else {
        let mbi = structures::MemoryBasicInformation64::guess(addr, &mut emu.maps);
        mbi.save(out_buff, &mut emu.maps);
        emu.set_last_error(0);
        emu.regs_mut().rax = structures::MemoryBasicInformation64::SIZE;
    }
}
