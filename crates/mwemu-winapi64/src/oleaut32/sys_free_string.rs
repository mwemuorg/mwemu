use crate::emu::Emu;
use crate::winapi::common::bstr;

pub fn SysFreeString(emu: &mut Emu) {
    bstr::free_api(emu);
}
