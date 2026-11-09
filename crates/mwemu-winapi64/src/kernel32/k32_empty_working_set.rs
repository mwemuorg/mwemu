use crate::emu;
use crate::winapi::helper;

pub fn K32EmptyWorkingSet(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!K32EmptyWorkingSet hProcess:0x{:x}",
        h_process
    );
    let _ = helper::handler_exist(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
