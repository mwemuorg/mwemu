use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DnsFree(emu: &mut emu::Emu) {
    let data = arg(emu, 0) as u64;
    log_red!(emu, "dnsapi!DnsFree ptr:0x{:x}", data);
    stdcall_ret(emu, 2);
}
