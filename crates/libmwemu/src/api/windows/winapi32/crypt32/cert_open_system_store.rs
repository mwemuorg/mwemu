use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertOpenSystemStore(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
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
    let _hprov = arg(emu, 0);
    let subsystem = arg(emu, 1) as u64;

    let name = emu.maps.read_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreA subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 2);
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    let _hprov = arg(emu, 0);
    let subsystem = arg(emu, 1) as u64;

    let name = emu.maps.read_wide_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreW subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 2);
}
