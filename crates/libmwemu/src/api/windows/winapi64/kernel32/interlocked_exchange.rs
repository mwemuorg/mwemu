use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;

pub fn InterlockedExchange(emu: &mut emu::Emu, _addr: u64) {
    dispatch_shared("InterlockedExchange", "kernel32", emu);
}
