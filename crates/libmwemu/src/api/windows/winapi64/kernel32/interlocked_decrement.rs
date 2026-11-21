use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;

pub fn InterlockedDecrement(emu: &mut emu::Emu, _addr: u64) {
    dispatch_shared("InterlockedDecrement", "kernel32", emu);
}
