use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;

pub(super) fn _initterm_e(emu: &mut emu::Emu) {
    dispatch_shared("_initterm_e", "kernelbase", emu);
}
