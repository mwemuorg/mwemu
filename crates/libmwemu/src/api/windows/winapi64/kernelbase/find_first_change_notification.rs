use crate::emu;
use crate::winapi::helper;

pub fn FindFirstChangeNotificationA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("change");
    log_red!(
        emu,
        "kernelbase!FindFirstChangeNotificationA '{}' p1:0x{:x} p2:0x{:x} => 0x{:x}",
        s,
        p1,
        p2,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("change");
    log_red!(
        emu,
        "kernelbase!FindFirstChangeNotificationW '{}' p1:0x{:x} p2:0x{:x} => 0x{:x}",
        s,
        p1,
        p2,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
