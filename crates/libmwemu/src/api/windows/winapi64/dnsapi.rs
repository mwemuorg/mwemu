use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

const DNS_ERROR_RCODE_SERVER_FAILURE: u64 = 9002;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "DnsQuery_A" | "DnsQueryA" => DnsQuery_A(emu),
        "DnsQuery_W" | "DnsQueryW" => DnsQuery_W(emu),
        "DnsFree" => {
            let data = emu.regs().rcx;
            log_red!(emu, "dnsapi!DnsFree ptr:0x{:x}", data);
        }
        "DnsRecordListFree" => {
            let record = emu.regs().rcx;
            log_red!(emu, "dnsapi!DnsRecordListFree ptr:0x{:x}", record);
        }
        "DnsNameCompare_A" => {
            let name1 = emu.regs().rcx;
            let name2 = emu.regs().rdx;
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
        }
        "DnsNameCompare_W" => {
            let name1 = emu.regs().rcx;
            let name2 = emu.regs().rdx;
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
        }

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn DnsQuery_A(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let wtype = emu.regs().rdx;
    let options = emu.regs().r8;

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
        options
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
}

fn DnsQuery_W(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let wtype = emu.regs().rdx;
    let options = emu.regs().r8;

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
        options
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
}
