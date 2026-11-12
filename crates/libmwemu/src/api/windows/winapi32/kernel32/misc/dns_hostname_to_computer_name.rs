use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn DnsHostnameToComputerNameA(emu: &mut emu::Emu) {
    dns_hostname_to_computer_name(emu, "DnsHostnameToComputerNameA", false);
}

fn dns_hostname_to_computer_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let Hostname = WinApiAbi::arg(emu, 0) as u32;
    let ComputerName = WinApiAbi::arg(emu, 1) as u32;
    let _nSize = WinApiAbi::arg(emu, 2) as u32;

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
