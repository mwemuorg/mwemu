use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::winapi::policy::api_try;
use crate::{emu, windows::structures};

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_sysinfo = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!GetSystemInfo sysinfo: 0x{:x}", out_sysinfo);
    if api_try!(emu, call.arg(emu, 0)) != 0 && emu.maps.is_mapped(api_try!(emu, call.arg(emu, 0))) {
        emu.maps.write_word(api_try!(emu, call.arg(emu, 0)), 9);
        emu.maps
            .write_dword(api_try!(emu, call.arg(emu, 0)) + 4, 4096);
    }

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(out_sysinfo, &mut emu.maps);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
