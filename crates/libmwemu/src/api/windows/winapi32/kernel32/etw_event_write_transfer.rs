use crate::emu::Emu;
use crate::winapi::policy;
pub fn EtwEventWriteTransfer(emu: &mut Emu) {
    policy::unsupported(emu, "kernel32", "EtwEventWriteTransfer", emu.pc());
}
