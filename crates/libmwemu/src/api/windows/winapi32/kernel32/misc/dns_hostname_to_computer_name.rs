use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn DnsHostnameToComputerNameA(emu: &mut emu::Emu) {
    dns_hostname_to_computer_name(emu, "DnsHostnameToComputerNameA", false);
}

fn dns_hostname_to_computer_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let Hostname = api_try!(emu, call.arg_u32(emu, 0));
    let ComputerName = api_try!(emu, call.arg_u32(emu, 1));
    let _nSize = api_try!(emu, call.arg_u32(emu, 2));

    let Hostname = strings::read_text(emu, Hostname as u64, wide);
    let ComputerName = strings::read_text(emu, ComputerName as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", Hostname, ComputerName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn DnsHostnameToComputerNameW(emu: &mut emu::Emu) {
    dns_hostname_to_computer_name(emu, "DnsHostnameToComputerNameW", true);
}
