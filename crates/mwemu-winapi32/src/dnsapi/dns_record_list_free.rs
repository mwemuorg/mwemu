use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DnsRecordListFree(emu: &mut emu::Emu) {
    let record = arg(emu, 0) as u64;
    log_red!(emu, "dnsapi!DnsRecordListFree ptr:0x{:x}", record);
    stdcall_ret(emu, 2);
}
