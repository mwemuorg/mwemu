use crate::emu::Emu;
use crate::winapi::common::bstr;

pub fn SysAllocStringLen(emu: &mut Emu) {
    bstr::alloc_api(emu, true, false);
}
