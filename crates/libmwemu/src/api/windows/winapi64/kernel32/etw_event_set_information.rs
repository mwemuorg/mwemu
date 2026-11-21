use crate::emu::Emu;
use crate::winapi::policy;
pub fn EtwEventSetInformation(emu: &mut Emu) {
    policy::unsupported(emu, "kernel32", "EtwEventSetInformation", emu.pc());
}
