use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertOpenSystemStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    call.cleanup_args(emu, 2);
    log_red!(
        emu,
        "crypt32!CertOpenSystemStore arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    call.set_return(emu, helper::handler_create("CertOpenSystemStore"));
}

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    cert_open_system_store(emu, "CertOpenSystemStoreA", false);
}

fn cert_open_system_store(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _hprov = api_try!(emu, call.arg_u32(emu, 0));
    let subsystem = api_try!(emu, call.arg_u32(emu, 1)) as u64;

    let name = strings::read_text(emu, subsystem, wide);

    log_red!(emu, "crypt32!{api} subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    call.set_return(emu, handle);
    call.cleanup_args(emu, 2);
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    cert_open_system_store(emu, "CertOpenSystemStoreW", true);
}
