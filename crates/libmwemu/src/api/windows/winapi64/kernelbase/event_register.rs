use crate::emu::Emu;
use crate::winapi::policy;
pub fn EventRegister(emu: &mut Emu) {
    policy::unsupported(emu, "kernelbase", "EventRegister", emu.pc());
}
