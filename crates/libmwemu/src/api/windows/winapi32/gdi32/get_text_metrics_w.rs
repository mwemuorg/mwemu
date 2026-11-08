use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTextMetricsW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let tm = arg(emu, 1) as u64;
    if tm != 0 {
        emu.maps.write_dword(tm, 16);
        emu.maps.write_dword(tm + 4, 13);
        emu.maps.write_dword(tm + 8, 3);
        emu.maps.write_dword(tm + 12, 0);
        emu.maps.write_dword(tm + 16, 0);
        emu.maps.write_dword(tm + 20, 7);
        emu.maps.write_dword(tm + 24, 14);
    }
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetTextMetricsW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
