use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!GetStringTypeA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    stdcall_ret(emu, 5);
    emu.regs_mut().rax = 1;
}

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let info_type = WinApiAbi::arg(emu, 0) as u32;
    let str_ptr = WinApiAbi::arg(emu, 1);
    let sz = WinApiAbi::arg(emu, 2) as u32;
    let char_type = WinApiAbi::arg(emu, 3) as u32;

    let ustr = emu.maps.read_wide_string(str_ptr);

    log_red!(emu, "kernel32!GetStringTypeW `{}` 0x{}", ustr, sz);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
