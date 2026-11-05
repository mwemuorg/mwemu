use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;
//use crate::winapi::helper;
//use crate::endpoint;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "DnsQuery_A" => DnsQuery_A(emu),
        "DnsQueryA" => DnsQuery_A(emu),
        "DnsQuery_W" => DnsQuery_W(emu),
        "DnsQueryW" => DnsQuery_W(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn DnsQuery_A(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let opt = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let extra = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let out_results = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let out_reserved = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "dnsapi!DnsQuery_A '{}'", name);

    emu.regs_mut().rax = 1;
}

fn DnsQuery_W(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let opt = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let extra = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let out_results = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let out_reserved = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let name = emu.maps.read_wide_string(name_ptr);

    log_red!(emu, "dnsapi!DnsQuery_W '{}'", name);

    emu.regs_mut().rax = 1;
}
