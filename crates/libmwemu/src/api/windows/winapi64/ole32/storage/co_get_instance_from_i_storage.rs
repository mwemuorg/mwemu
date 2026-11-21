use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoGetInstanceFromIStorage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let clsid_ptr = api_try!(emu, call.arg(emu, 0));
    let outer = api_try!(emu, call.arg(emu, 1));
    let ctx = api_try!(emu, call.arg(emu, 2));
    let iid_ptr = api_try!(emu, call.arg(emu, 3));
    let ppv = api_try!(emu, call.arg(emu, 5));
    let hndl = helper::handler_create("com_object");
    let ppv_addr = ppv;
    if ppv_addr != 0 {
        emu.maps.write_qword(ppv_addr, hndl);
    }
    log_red!(
        emu,
        "ole32!CoGetInstanceFromIStorage ctx:0x{:x} => 0x{:x}",
        ctx,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
