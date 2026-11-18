use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn DnsNameCompare_A(emu: &mut emu::Emu) {
    dns_name_compare_(emu, "DnsNameCompare_A", false);
}

fn dns_name_compare_(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name1 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let name2 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let s1 = if name1 != 0 {
        strings::read_text(emu, name1, wide)
    } else {
        String::new()
    };
    let s2 = if name2 != 0 {
        strings::read_text(emu, name2, wide)
    } else {
        String::new()
    };
    log_red!(emu, "dnsapi!{api} '{}' vs '{}'", s1, s2);
    call.set_return(emu, if s1.eq_ignore_ascii_case(&s2) { 1 } else { 0 });
    call.cleanup_args(emu, 2);
}

pub fn DnsNameCompare_W(emu: &mut emu::Emu) {
    dns_name_compare_(emu, "DnsNameCompare_W", true);
}
