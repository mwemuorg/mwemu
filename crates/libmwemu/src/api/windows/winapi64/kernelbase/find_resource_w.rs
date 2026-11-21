use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindResourceW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let module = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let type_ptr = api_try!(emu, call.arg(emu, 2));
    let name = if name_ptr > 0xffff {
        emu.maps.read_wide_string(name_ptr)
    } else {
        format!("#{}", name_ptr)
    };
    let hndl = helper::handler_create(&format!("resource_{}", name));
    log_red!(
        emu,
        "kernelbase!FindResourceW module:0x{:x} '{}' => 0x{:x}",
        module,
        name,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
