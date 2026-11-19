use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LogonUserA(emu: &mut emu::Emu) {
    logon_user(emu, "LogonUserA", false);
}

fn logon_user(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let username_ptr = api_try!(emu, call.arg(emu, 0));
    let domain_ptr = api_try!(emu, call.arg(emu, 1));
    let _password_ptr = api_try!(emu, call.arg(emu, 2));
    let _logon_type = api_try!(emu, call.arg(emu, 3));
    let _logon_provider = api_try!(emu, call.arg(emu, 4));
    let token_ptr = api_try!(emu, call.arg(emu, 5));
    let user = if username_ptr != 0 && username_ptr > 0xffff {
        strings::read_text(emu, username_ptr, wide)
    } else {
        String::from("unknown")
    };
    let domain = if domain_ptr != 0 && domain_ptr > 0xffff {
        strings::read_text(emu, domain_ptr, wide)
    } else {
        String::from(".")
    };
    let token = helper::handler_create("logon_token");
    if token_ptr != 0 {
        emu.maps.write_qword(token_ptr, token);
    }
    log_red!(
        emu,
        "advapi32!{api} user:'{}' domain:'{}' => token:0x{:x}",
        user,
        domain,
        token
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn LogonUserW(emu: &mut emu::Emu) {
    logon_user(emu, "LogonUserW", true);
}
