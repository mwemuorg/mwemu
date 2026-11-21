use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DnsRecordListFree(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let record = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "dnsapi!DnsRecordListFree ptr:0x{:x}", record);
    call.cleanup_args(emu, 2);
}
