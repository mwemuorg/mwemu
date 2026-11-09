use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    let _hprov = arg(emu, 0);
    let subsystem = arg(emu, 1) as u64;

    let name = emu.maps.read_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreA subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 2);
}
