use crate::emu::Emu;
use crate::winapi::common::bstr;

pub(super) fn SysAllocString(emu: &mut Emu) {
    bstr::alloc_api(emu, false, false);
}
