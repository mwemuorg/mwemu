use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::winapi::policy::api_try;
use crate::{emu, windows::structures};

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr_sysinfo = api_try!(emu, call.arg(emu, 0));

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(ptr_sysinfo, &mut emu.maps);

    log_red!(emu, "kernel32!GetNativeSysteminfo {:?}", sysinfo);
    if api_try!(emu, call.arg(emu, 0)) != 0 && emu.maps.is_mapped(api_try!(emu, call.arg(emu, 0))) {
        emu.maps.write_word(api_try!(emu, call.arg(emu, 0)), 9);
        emu.maps
            .write_dword(api_try!(emu, call.arg(emu, 0)) + 4, 4096);
    }

    log_red!(emu, "kernel32!GetNativeSysteminfo 0x{:x}", ptr_sysinfo);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
