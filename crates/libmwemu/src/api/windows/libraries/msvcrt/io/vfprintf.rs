use crate::emu::Emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::formatted_print;

pub fn vfprintf(emu: &mut Emu) {
    formatted_print(emu, Runtime::Msvcrt, "vfprintf");
}
