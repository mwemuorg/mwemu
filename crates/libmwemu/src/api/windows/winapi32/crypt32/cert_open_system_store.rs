use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertOpenSystemStore(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertOpenSystemStore arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = helper::handler_create("CertOpenSystemStore");
}

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    cert_open_system_store(emu, "CertOpenSystemStoreA", false);
}

fn cert_open_system_store(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hprov = arg(emu, 0);
    let subsystem = arg(emu, 1) as u64;

    let name = strings::read_text(emu, subsystem, wide);

    log_red!(emu, "crypt32!{api} subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 2);
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    cert_open_system_store(emu, "CertOpenSystemStoreW", true);
}
