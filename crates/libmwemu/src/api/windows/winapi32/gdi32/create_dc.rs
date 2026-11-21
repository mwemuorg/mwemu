use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDCA(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCA", false);
}

pub fn CreateDCW(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCW", true);
}

fn create_dc(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    if wide {
    } else {
        let driver = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    }

    let device = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let name = if device != 0 {
        strings::read_text(emu, device, wide)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!{api} device:'{}'", name);
    let hndl = helper::handler_create(&format!("gdi32://{api}"));
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 4);
}
