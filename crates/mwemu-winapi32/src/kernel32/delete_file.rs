use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    let filename_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let filename = emu.maps.read_string(filename_ptr);

    log_red!(
        emu,
        "kernel32!DeleteFileA `{}` (stub: not deleting)",
        filename
    );

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1; // Success
}

pub fn DeleteFileW(emu: &mut emu::Emu) {
    let filename = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(
        emu,
        "kernel32!DeleteFileW `{}` (stub: not deleting)",
        filename
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
