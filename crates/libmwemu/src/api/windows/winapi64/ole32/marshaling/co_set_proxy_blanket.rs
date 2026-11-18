use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CoSetProxyBlanket(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let proxy = api_try!(emu, call.arg(emu, 0));
    let authn_svc = api_try!(emu, call.arg(emu, 1));
    let authz_svc = api_try!(emu, call.arg(emu, 2));
    let server_princ = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ole32!CoSetProxyBlanket proxy:0x{:x} authn:{} authz:{}",
        proxy,
        authn_svc,
        authz_svc
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
    for _ in 0..8 {}
}
