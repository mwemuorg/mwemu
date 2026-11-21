use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateMutexExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "kernelbase!CreateMutexExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    let hndl = helper::handler_create("mutex://CreateMutexExA");
    call.set_return(emu, hndl);
}

pub fn CreateMutexExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let security = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let access = api_try!(emu, call.arg(emu, 3));
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("mutex_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateMutexExW '{}' flags:0x{:x} => 0x{:x}",
        name,
        flags,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
