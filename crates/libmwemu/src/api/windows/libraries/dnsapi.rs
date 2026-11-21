use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::{memory, trace};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;

fn compare(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [first, second] = api_try!(emu, call.args(emu));
    let key = |address| {
        let units = encoding.read(emu, address).ok()?;
        let text = String::from_utf16(&units).ok()?;
        Some(text.trim_end_matches('.').to_lowercase())
    };
    let equal = key(first)
        .zip(key(second))
        .is_some_and(|(first, second)| first == second);
    log_red!(
        emu,
        "dnsapi!{api} name1={name1} name2={name2} equal={equal}",
        name1 = trace::input_string(emu, first, encoding),
        name2 = trace::input_string(emu, second, encoding)
    );
    call.set_return(emu, u64::from(equal));
    call.cleanup_args(emu, 2);
}

pub fn DnsNameCompare_A(emu: &mut Emu) {
    compare(emu, "DnsNameCompare_A", Encoding::Ansi);
}

pub fn DnsNameCompare_W(emu: &mut Emu) {
    compare(emu, "DnsNameCompare_W", Encoding::Wide);
}

pub(crate) fn query(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [name, record_type, options, servers, records, reserved] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "dnsapi!{api} name={name} record_type={record_type} options={options:#x} servers={servers:#x} records={records:#x} reserved={reserved:#x}",
        name = trace::input_string(emu, name, encoding),
        record_type = record_type as u16
    );
    let status =
        if records == 0 || !memory::write_pointer(emu, records, 0) || name == 0 || reserved != 0 {
            ERROR_INVALID_PARAMETER
        } else {
            DNS_ERROR_RCODE_SERVER_FAILURE
        };
    call.set_return(emu, status);
    call.cleanup_args(emu, 6);
}

fn free(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [data, free_type] = api_try!(emu, call.args(emu));
    if data != 0 {
        policy::unsupported(emu, "dnsapi", api, emu.pc());
        return;
    }
    log_red!(emu, "dnsapi!{api} data=NULL free_type={free_type}");
    call.cleanup_args(emu, 2);
}

pub fn DnsFree(emu: &mut Emu) {
    free(emu, "DnsFree");
}

pub fn DnsRecordListFree(emu: &mut Emu) {
    free(emu, "DnsRecordListFree");
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "DnsQuery_A" | "DnsQueryA" => query(emu, api, Encoding::Ansi),
        "DnsQuery_W" | "DnsQueryW" => query(emu, api, Encoding::Wide),
        "DnsFree" => DnsFree(emu),
        "DnsRecordListFree" => DnsRecordListFree(emu),
        "DnsNameCompare_A" => DnsNameCompare_A(emu),
        "DnsNameCompare_W" => DnsNameCompare_W(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
