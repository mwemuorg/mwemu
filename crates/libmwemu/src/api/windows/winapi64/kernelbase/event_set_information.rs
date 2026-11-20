use crate::emu::Emu;
use crate::winapi::policy;
pub fn EventSetInformation(emu: &mut Emu) {
    policy::unsupported(emu, "kernelbase", "EventSetInformation", emu.pc());
}
