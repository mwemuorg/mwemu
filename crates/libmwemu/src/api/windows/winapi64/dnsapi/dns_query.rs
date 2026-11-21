use super::DNS_ERROR_RCODE_SERVER_FAILURE;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn DnsQuery_A(emu: &mut emu::Emu) {
    dns_query_(emu, "DnsQuery_A", false);
}

fn dns_query_(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let wtype = api_try!(emu, call.arg(emu, 1));
    let options = api_try!(emu, call.arg(emu, 2));

    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "dnsapi!{api} name:'{}' type:{} options:0x{:x}",
        name,
        wtype,
        options
    );

    call.set_return(emu, DNS_ERROR_RCODE_SERVER_FAILURE);
}

pub(super) fn DnsQuery_W(emu: &mut emu::Emu) {
    dns_query_(emu, "DnsQuery_W", true);
}
