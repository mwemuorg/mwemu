use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn LCMapStringA(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let p5 = WinApiAbi::arg(emu, 5);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!LCMapStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn LCMapStringW(emu: &mut emu::Emu) {
    let locale = WinApiAbi::arg(emu, 0) as u32;
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let src_ptr = WinApiAbi::arg(emu, 2);
    let src_sz = WinApiAbi::arg(emu, 3) as u32;
    let dest_ptr = WinApiAbi::arg(emu, 4);
    let dest_sz = WinApiAbi::arg(emu, 5) as u32;

    let s = emu.maps.read_wide_string(src_ptr);
    let char_count = if src_sz as i32 == -1 {
        s.len() as u32 + 1
    } else {
        src_sz
    };

    log_red!(
        emu,
        "kernel32!LCMapStringW `{}` dst:0x{:x} sz:{}->{}",
        s,
        dest_ptr,
        src_sz,
        dest_sz
    );

    if dest_ptr > 0 && dest_sz > 0 {
        emu.maps.write_wide_string(dest_ptr, &s);
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = char_count as u64;
}
