use crate::emu::Emu;
use crate::winapi::policy;
pub fn EtwEventRegister(emu: &mut Emu) {
    policy::unsupported(emu, "kernel32", "EtwEventRegister", emu.pc());
}
