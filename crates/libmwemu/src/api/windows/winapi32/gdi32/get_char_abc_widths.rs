use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetCharABCWidthsA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            let off = (i as u64) * 12; // ABC struct: 3 ints
            emu.maps.write_dword(buf + off, 1); // abcA
            emu.maps.write_dword(buf + off + 4, 6); // abcB
            emu.maps.write_dword(buf + off + 8, 1); // abcC
        }
    }
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetCharABCWidthsA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn GetCharABCWidthsW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            let off = (i as u64) * 12;
            emu.maps.write_dword(buf + off, 1);
            emu.maps.write_dword(buf + off + 4, 6);
            emu.maps.write_dword(buf + off + 8, 1);
        }
    }
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetCharABCWidthsW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
