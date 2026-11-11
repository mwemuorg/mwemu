use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod dns_free;
pub use dns_free::DnsFree;
mod dns_record_list_free;
pub use dns_record_list_free::DnsRecordListFree;
mod dns_name_compare;
pub use dns_name_compare::DnsNameCompare_A;
pub use dns_name_compare::DnsNameCompare_W;
mod dns_query;
use dns_query::DnsQuery_A;
use dns_query::DnsQuery_W;

const DNS_ERROR_RCODE_SERVER_FAILURE: u64 = 9002;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "DnsQuery_A" | "DnsQueryA" => DnsQuery_A(emu),
        "DnsQuery_W" | "DnsQueryW" => DnsQuery_W(emu),
        "DnsFree" => DnsFree(emu),
        "DnsRecordListFree" => DnsRecordListFree(emu),
        "DnsNameCompare_A" => DnsNameCompare_A(emu),
        "DnsNameCompare_W" => DnsNameCompare_W(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
