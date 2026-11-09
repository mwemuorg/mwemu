use crate::emu;
use crate::winapi::helper;

pub fn MapViewOfFileEx(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(
        emu,
        "kernelbase!MapViewOfFileEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let handle = helper::handler_create("MapViewOfFileEx");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x100000;
}
