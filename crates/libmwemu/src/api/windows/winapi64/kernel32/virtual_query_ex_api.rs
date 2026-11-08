use crate::emu;
use crate::windows::structures;

pub fn VirtualQueryEx_api(emu: &mut emu::Emu, addr: u64) {
    let _process = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let out_buff = emu.regs().r8;
    let size = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!VirtualQueryEx 0x{:x} 0x{:x} {}",
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
