use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;

pub fn InterlockedIncrement(emu: &mut emu::Emu, _addr: u64) {
    dispatch_shared("InterlockedIncrement", "kernel32", emu);
}
