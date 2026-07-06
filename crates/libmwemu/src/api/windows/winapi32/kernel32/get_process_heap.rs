use crate::emu;
use crate::winapi::helper;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    emu.regs_mut().rax = helper::handler_create("process heap");
    log_red!(
        emu,
        "kernel32!GetProcessHeap =0x{:x}",
        emu.regs().rax as u32
    );
}
