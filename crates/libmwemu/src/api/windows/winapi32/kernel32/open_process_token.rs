use crate::emu;
use crate::winapi::helper;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!OpenProcessToken error reading param");
    let access = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!OpenProcessToken error reading param");
    let ptr_token = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernel32!OpenProcessToken error reading param") as u64;

    log_red!(emu, "kernel32!OpenProcessToken 0x{:x} {}", hndl, access);

    emu.maps.write_dword(
        ptr_token,
        helper::handler_create(&format!("token://{}", hndl)) as u32,
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
