use crate::emu::Emu;
use crate::winapi::policy;
pub fn EventWriteTransfer(emu: &mut Emu) {
    policy::unsupported(emu, "kernelbase", "EventWriteTransfer", emu.pc());
}
