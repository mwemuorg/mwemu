use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

const DNS_ERROR_RCODE_SERVER_FAILURE: u64 = 9002;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "DnsQuery_A" | "DnsQueryA" => DnsQuery_A(emu),
        "DnsQuery_W" | "DnsQueryW" => DnsQuery_W(emu),
        "DnsFree" => {
            let data = arg(emu, 0) as u64;
            log_red!(emu, "dnsapi!DnsFree ptr:0x{:x}", data);
            stdcall_ret(emu, 2);
        }
        "DnsRecordListFree" => {
            let record = arg(emu, 0) as u64;
            log_red!(emu, "dnsapi!DnsRecordListFree ptr:0x{:x}", record);
            stdcall_ret(emu, 2);
        }
        "DnsNameCompare_A" => {
            let name1 = arg(emu, 0) as u64;
            let name2 = arg(emu, 1) as u64;
            let s1 = if name1 != 0 {
                emu.maps.read_string(name1)
            } else {
                String::new()
            };
            let s2 = if name2 != 0 {
                emu.maps.read_string(name2)
            } else {
                String::new()
            };
            log_red!(emu, "dnsapi!DnsNameCompare_A '{}' vs '{}'", s1, s2);
            emu.regs_mut().rax = if s1.eq_ignore_ascii_case(&s2) { 1 } else { 0 };
            stdcall_ret(emu, 2);
        }
        "DnsNameCompare_W" => {
            let name1 = arg(emu, 0) as u64;
            let name2 = arg(emu, 1) as u64;
            let s1 = if name1 != 0 {
                emu.maps.read_wide_string(name1)
            } else {
                String::new()
            };
            let s2 = if name2 != 0 {
                emu.maps.read_wide_string(name2)
            } else {
                String::new()
            };
            log_red!(emu, "dnsapi!DnsNameCompare_W '{}' vs '{}'", s1, s2);
            emu.regs_mut().rax = if s1.eq_ignore_ascii_case(&s2) { 1 } else { 0 };
            stdcall_ret(emu, 2);
        }

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}

fn DnsQuery_A(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let wtype = arg(emu, 1);
    let opt = arg(emu, 2);

    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "dnsapi!DnsQuery_A name:'{}' type:{} options:0x{:x}",
        name,
        wtype,
        opt
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
    stdcall_ret(emu, 6);
}

fn DnsQuery_W(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let wtype = arg(emu, 1);
    let opt = arg(emu, 2);

    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "dnsapi!DnsQuery_W name:'{}' type:{} options:0x{:x}",
        name,
        wtype,
        opt
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
    stdcall_ret(emu, 6);
}
