use crate::emu;
use crate::winapi::helper;

pub fn Process32First(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let lppe = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Process32First hndl:0x{:x} entry:0x{:x}",
        handle,
        lppe
    );

    if !helper::handler_exist(handle) {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    if lppe == 0 {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
        return;
    }

    // PROCESSENTRY32 (on 64-bit, offsets are slightly different due to ULONG_PTR)
    // dwSize at +0 (already set by caller)
    // cntUsage at +4
    emu.maps.write_dword(lppe + 4, 0);
    // th32ProcessID at +8
    emu.maps.write_dword(lppe + 8, 4); // System process
    // th32DefaultHeapID at +16 (ULONG_PTR = 8 bytes on 64-bit)
    emu.maps.write_qword(lppe + 16, 0);
    // th32ModuleID at +24
    emu.maps.write_dword(lppe + 24, 0);
    // cntThreads at +28
    emu.maps.write_dword(lppe + 28, 2);
    // th32ParentProcessID at +32
    emu.maps.write_dword(lppe + 32, 0);
    // pcPriClassBase at +36
    emu.maps.write_dword(lppe + 36, 8); // NORMAL_PRIORITY_CLASS
    // dwFlags at +40
    emu.maps.write_dword(lppe + 40, 0);
    // szExeFile at +44 (MAX_PATH=260 chars)
    emu.maps.write_string(lppe + 44, "smss.exe");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
