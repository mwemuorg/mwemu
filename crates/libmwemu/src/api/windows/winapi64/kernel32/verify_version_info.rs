use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    verify_version_info(emu, "VerifyVersionInfoA");
}

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    verify_version_info(emu, "VerifyVersionInfoW");
}

fn verify_version_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let version_info = api_try!(emu, call.arg(emu, 0));
    let type_mask = api_try!(emu, call.arg(emu, 1));
    let condition_mask = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "kernel32!{} info:0x{:x} type_mask:0x{:x} cond_mask:0x{:x} -> TRUE",
        api,
        version_info,
        type_mask,
        condition_mask
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
