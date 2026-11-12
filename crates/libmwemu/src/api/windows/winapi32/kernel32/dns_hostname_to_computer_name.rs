use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn DnsHostnameToComputerNameA(emu: &mut emu::Emu) {
    dns_hostname_to_computer_name(emu, "DnsHostnameToComputerNameA", false);
}

fn dns_hostname_to_computer_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let Hostname = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let ComputerName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let Hostname = strings::read_text(emu, Hostname as u64, wide);
    let ComputerName = strings::read_text(emu, ComputerName as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", Hostname, ComputerName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn DnsHostnameToComputerNameW(emu: &mut emu::Emu) {
    dns_hostname_to_computer_name(emu, "DnsHostnameToComputerNameW", true);
}
