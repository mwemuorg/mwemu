use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "kernelbase!VirtualProtectEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
