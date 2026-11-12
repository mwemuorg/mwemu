use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0) as u32;
    let access = WinApiAbi::arg(emu, 1) as u32;
    let ptr_token = WinApiAbi::arg(emu, 2);

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
