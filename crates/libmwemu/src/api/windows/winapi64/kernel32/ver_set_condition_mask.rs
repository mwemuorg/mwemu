use crate::emu::Emu;
use crate::winapi::common::ntdll;

pub fn VerSetConditionMask(emu: &mut Emu) {
    ntdll::version_condition_mask(emu);
}
