use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DllGetActivationFactory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let class_id_ptr = api_try!(emu, call.arg(emu, 0));
    let iid_ptr = api_try!(emu, call.arg(emu, 1));
    let factory_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "shell32!DllGetActivationFactory");
    emu.set_last_error(0);
    call.set_return(emu, 0x80004001u64);
}
