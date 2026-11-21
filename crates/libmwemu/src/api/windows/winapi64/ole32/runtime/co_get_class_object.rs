use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoGetClassObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let clsid_ptr = api_try!(emu, call.arg(emu, 0));
    let ctx = api_try!(emu, call.arg(emu, 1));
    let server_info = api_try!(emu, call.arg(emu, 2));
    let iid_ptr = api_try!(emu, call.arg(emu, 3));
    let ppv = api_try!(emu, call.arg(emu, 4));
    let hndl = helper::handler_create("com_class_factory");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(
        emu,
        "ole32!CoGetClassObject ctx:0x{:x} => 0x{:x}",
        ctx,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
