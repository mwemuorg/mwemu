use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetSecurityInfo(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!SetSecurityInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    stdcall_ret(emu, 7);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
