use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::winapi::policy::api_try;
use crate::{emu, windows::structures};

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_time = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!GetSystemTime ptr: 0x{:x}'", out_time);
    if api_try!(emu, call.arg(emu, 0)) != 0 && emu.maps.is_mapped(api_try!(emu, call.arg(emu, 0))) {
        for i in 0..8u64 {
            emu.maps
                .write_word(api_try!(emu, call.arg(emu, 0)) + i * 2, 0);
        }
    }

    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
