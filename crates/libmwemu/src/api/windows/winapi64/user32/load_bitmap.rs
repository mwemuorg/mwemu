use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadBitmapA(emu: &mut emu::Emu) {
    load_bitmap(emu, "LoadBitmapA");
}

fn load_bitmap(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hinst = api_try!(emu, call.arg(emu, 0));
    let name = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!{api} hInst:0x{:x} name:0x{:x}", hinst, name);
    let hndl = helper::handler_create("user32://bitmap");
    call.set_return(emu, hndl);
}

pub fn LoadBitmapW(emu: &mut emu::Emu) {
    load_bitmap(emu, "LoadBitmapW");
}
