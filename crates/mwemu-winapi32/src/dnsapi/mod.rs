use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod dns_free;
pub use dns_free::DnsFree;
mod dns_record_list_free;
pub use dns_record_list_free::DnsRecordListFree;
mod dns_name_compare_a;
pub use dns_name_compare_a::DnsNameCompare_A;
mod dns_name_compare_w;
pub use dns_name_compare_w::DnsNameCompare_W;
mod dns_query_a;
use dns_query_a::DnsQuery_A;
mod dns_query_w;
use dns_query_w::DnsQuery_W;

const DNS_ERROR_RCODE_SERVER_FAILURE: u64 = 9002;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "DnsQuery_A" | "DnsQueryA" => DnsQuery_A(emu),
        "DnsQuery_W" | "DnsQueryW" => DnsQuery_W(emu),
        "DnsFree" => DnsFree(emu),
        "DnsRecordListFree" => DnsRecordListFree(emu),
        "DnsNameCompare_A" => DnsNameCompare_A(emu),
        "DnsNameCompare_W" => DnsNameCompare_W(emu),

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}
