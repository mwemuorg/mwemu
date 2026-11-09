use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn GdiChord(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
    let arg7 = emu.maps.read_dword(esp + 28).unwrap_or(0) as u64;
    let arg8 = emu.maps.read_dword(esp + 32).unwrap_or(0) as u64;
    let arg9 = emu.maps.read_dword(esp + 36).unwrap_or(0) as u64;
    for _ in 0..9u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!Chord arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x} arg8:0x{:x} arg9:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}
