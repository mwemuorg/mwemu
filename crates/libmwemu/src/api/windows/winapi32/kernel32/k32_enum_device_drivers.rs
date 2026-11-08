use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn K32EnumDeviceDrivers(emu: &mut emu::Emu) {
    let p_image_base = arg(emu, 0) as u64;
    let cb = arg(emu, 1) as u64;
    let p_needed = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!K32EnumDeviceDrivers pImageBase:0x{:x} cb:{} pNeeded:0x{:x}",
        p_image_base,
        cb,
        p_needed
    );

    if p_needed != 0 {
        emu.maps.write_dword(p_needed, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
