use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoRegisterClassObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rclsid = api_try!(emu, call.arg(emu, 0));
    let punk = api_try!(emu, call.arg(emu, 1));
    let cls_ctx = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    let pdw_register = api_try!(emu, call.arg(emu, 4));
    let hndl = helper::handler_create("com_class_registration");
    if pdw_register != 0 {
        emu.maps.write_dword(pdw_register, hndl as u32);
    }
    log_red!(
        emu,
        "ole32!CoRegisterClassObject clsid:0x{:x} ctx:{} => 0x{:x}",
        rclsid,
        cls_ctx,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
