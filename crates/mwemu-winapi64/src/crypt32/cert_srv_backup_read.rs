use crate::emu;
use crate::winapi::helper;

pub fn CertSrvBackupRead(emu: &mut emu::Emu) {
    let hbc = emu.regs().rcx;
    log_red!(emu, "crypt32!CertSrvBackupRead hbc:0x{:x}", hbc);
    let result = if helper::handler_exist(hbc) {
        0u64
    } else {
        1u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
