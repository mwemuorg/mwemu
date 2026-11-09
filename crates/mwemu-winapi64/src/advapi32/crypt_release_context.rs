use crate::emu;
use crate::winapi::helper;

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;

    log_red!(emu, "advapi32!CryptReleaseContext hProv=0x{:x}", hprov);

    helper::handler_close(hprov);

    emu.regs_mut().rax = 1;
}
