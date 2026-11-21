use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn internet_open_a(emu: &mut emu::Emu) {
    internet_open(emu, "InternetOpenA", false);
}

pub(super) fn internet_open_w(emu: &mut emu::Emu) {
    internet_open(emu, "InternetOpenW", true);
}

fn internet_open(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let uagent_ptr = api_try!(emu, call.arg(emu, 0));
    let _access = api_try!(emu, call.arg(emu, 1));
    let proxy_ptr = api_try!(emu, call.arg(emu, 2));
    let proxybypass_ptr = api_try!(emu, call.arg(emu, 3));
    let _flags = api_try!(emu, call.arg_u32(emu, 4));

    let uagent = strings::read_text(emu, uagent_ptr, wide);
    let proxy = strings::read_text(emu, proxy_ptr, wide);
    let proxy_bypass = strings::read_text(emu, proxybypass_ptr, wide);

    log_red!(
        emu,
        "wininet!{api} uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    call.cleanup_args(emu, 5);

    let uri = if wide {
        api.to_owned()
    } else {
        format!("uagent://{uagent}")
    };
    call.set_return(emu, helper::handler_create(&uri));
}

pub(super) fn internet_connect_a(emu: &mut emu::Emu) {
    internet_connect(emu, "InternetConnectA", false);
}

pub(super) fn internet_connect_w(emu: &mut emu::Emu) {
    internet_connect(emu, "InternetConnectW", true);
}

fn internet_connect(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let internet_hndl = api_try!(emu, call.arg(emu, 0));
    let server_ptr = api_try!(emu, call.arg(emu, 1));
    let port = api_try!(emu, call.arg_u32(emu, 2));
    let login_ptr = api_try!(emu, call.arg(emu, 3));
    let passw_ptr = api_try!(emu, call.arg(emu, 4));
    let _service = api_try!(emu, call.arg_u32(emu, 5));
    let _flags = api_try!(emu, call.arg_u32(emu, 6));
    let _ctx = api_try!(emu, call.arg_u32(emu, 7));

    let server = strings::read_text(emu, server_ptr, wide);
    let login = strings::read_text(emu, login_ptr, wide);
    let passw = strings::read_text(emu, passw_ptr, wide);

    log_red!(
        emu,
        "wininet!{api} host: {} port: {} login: {} passw: {}",
        server,
        port,
        login,
        passw
    );

    if !helper::handler_exist(internet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    call.cleanup_args(emu, 8);

    let uri = if wide {
        format!("{api}://{server}:{port}")
    } else {
        format!("{api}://{server}")
    };
    call.set_return(emu, helper::handler_create(&uri));
}
