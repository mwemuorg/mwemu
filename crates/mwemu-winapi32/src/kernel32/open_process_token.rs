use crate::emu;
use crate::winapi::helper;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let access = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let ptr_token = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0) as u64;

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
