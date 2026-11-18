use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFile2FromAppW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let access = api_try!(emu, call.arg(emu, 1));
    let share = api_try!(emu, call.arg(emu, 2));
    let disposition = api_try!(emu, call.arg_u32(emu, 3));
    let extended_parameters = api_try!(emu, call.arg(emu, 4));
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&format!("file_{}", path));
    log_red!(
        emu,
        "kernelbase!CreateFile2FromAppW '{}' access:0x{:x} disp:{} parameters:0x{:x} => 0x{:x}",
        path,
        access,
        disposition,
        extended_parameters,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
