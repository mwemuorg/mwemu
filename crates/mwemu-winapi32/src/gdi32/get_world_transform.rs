use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetWorldTransform(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let xform = arg(emu, 1) as u64;
    if xform != 0 {
        // Identity matrix: eM11=1.0, eM12=0, eM21=0, eM22=1.0, eDx=0, eDy=0
        emu.maps.write_dword(xform, 0x3f80_0000); // 1.0f
        emu.maps.write_dword(xform + 4, 0);
        emu.maps.write_dword(xform + 8, 0);
        emu.maps.write_dword(xform + 12, 0x3f80_0000); // 1.0f
        emu.maps.write_dword(xform + 16, 0);
        emu.maps.write_dword(xform + 20, 0);
    }
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetWorldTransform arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
