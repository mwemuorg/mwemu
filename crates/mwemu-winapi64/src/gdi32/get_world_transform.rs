use crate::emu;

pub fn GetWorldTransform(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let xform = emu.regs().rdx;
    if xform != 0 {
        // Identity matrix: eM11=1.0f, eM12=0, eM21=0, eM22=1.0f, eDx=0, eDy=0
        emu.maps.write_dword(xform, 0x3f80_0000); // 1.0f eM11
        emu.maps.write_dword(xform + 4, 0); // eM12
        emu.maps.write_dword(xform + 8, 0); // eM21
        emu.maps.write_dword(xform + 12, 0x3f80_0000); // 1.0f eM22
        emu.maps.write_dword(xform + 16, 0); // eDx
        emu.maps.write_dword(xform + 20, 0); // eDy
    }
    log_red!(
        emu,
        "gdi32!GetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        xform
    );
    emu.regs_mut().rax = 1;
}
