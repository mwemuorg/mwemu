use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;

pub fn WakeConditionVariable(emu: &mut emu::Emu, api: &str) {
    dispatch_shared(api, "kernel32", emu);
}
