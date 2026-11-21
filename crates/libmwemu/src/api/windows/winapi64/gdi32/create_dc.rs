use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDCA(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCA", false);
}

fn create_dc(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let driver_ptr = api_try!(emu, call.arg(emu, 0));
    let device_ptr = api_try!(emu, call.arg(emu, 1));
    let driver_name = if driver_ptr != 0 {
        strings::read_text(emu, driver_ptr, wide)
    } else {
        String::new()
    };
    let device_name = if device_ptr != 0 {
        strings::read_text(emu, device_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} driver:'{}' device:'{}'",
        driver_name,
        device_name
    );
    call.set_return(emu, helper::handler_create("gdi32://dc"));
}

pub fn CreateDCW(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCW", true);
}
