use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegisterTypeLibForUser(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = emu.maps.read_string(path_ptr);
    let hndl = helper::handler_create(&format!("typelib_{}", path));
    log_red!(
        emu,
        "oleaut32!RegisterTypeLibForUser '{}'  => 0x{:x}",
        path,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
