use crate::emu::Emu;
use crate::winapi::policy;
pub fn EventUnregister(emu: &mut Emu) {
    policy::unsupported(emu, "kernelbase", "EventUnregister", emu.pc());
}
