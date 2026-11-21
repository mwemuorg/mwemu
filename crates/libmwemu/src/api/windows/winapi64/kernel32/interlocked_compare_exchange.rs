use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu, _addr: u64) {
    dispatch_shared("InterlockedCompareExchange", "kernel32", emu);
}
