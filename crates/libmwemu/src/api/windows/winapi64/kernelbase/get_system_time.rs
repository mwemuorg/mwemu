use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!GetSystemTime ptr:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        emu.maps.write_word(p0, 2024);
        emu.maps.write_word(p0 + 2, 1);
        emu.maps.write_word(p0 + 4, 3);
        emu.maps.write_word(p0 + 6, 15);
        emu.maps.write_word(p0 + 8, 10);
        emu.maps.write_word(p0 + 10, 30);
        emu.maps.write_word(p0 + 12, 0);
        emu.maps.write_word(p0 + 14, 0);
    }
    emu.set_last_error(0);
}
