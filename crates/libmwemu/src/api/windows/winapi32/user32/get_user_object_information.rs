use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetUserObjectInformationA(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "user32!GetUserObjectInformationA arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub(super) fn GetUserObjectInformationW(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let nidx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let out_vinfo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let nlen = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let out_len = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "user32!GetUserObjectInformationW");

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1; // get handler
}
