use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetSecurityDescriptorDacl(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "advapi32!SetSecurityDescriptorDacl p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
