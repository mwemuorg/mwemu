use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::emu::Emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pFileName = api_try!(emu, call.arg(emu, 0));
    let bDeleteExistingResources = api_try!(emu, call.arg(emu, 1));

    let filename = emu.maps.read_string(pFileName);

    log_red!(
        emu,
        "kernel32!BeginUpdateResourceA `{}` {}",
        filename,
        bDeleteExistingResources
    );

    call.set_return(emu, helper::handler_create(&filename));
}

pub fn BeginUpdateResourceW(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let filename = emu.maps.read_wide_string(api_try!(emu, call.arg(emu, 0)));
    let delete_existing = api_try!(emu, call.arg(emu, 1)) != 0;
    log_red!(
        emu,
        "kernel32!BeginUpdateResourceW '{}' delete_existing:{}",
        filename,
        delete_existing
    );
    call.set_return(emu, helper::handler_create(&filename));
}
