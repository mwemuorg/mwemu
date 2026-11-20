use crate::emu::Emu;
use crate::winapi::common::process;

pub fn GetProcessAffinityMask(emu: &mut Emu) {
    process::process_masks(emu, "kernel32", "GetProcessAffinityMask");
}
