use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn Process32First(emu: &mut emu::Emu) {
    process32_first(emu, "Process32First", false);
}

pub fn Process32FirstW(emu: &mut emu::Emu) {
    process32_first(emu, "Process32FirstW", true);
}

fn process32_first(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
        let snap = emu.regs().rcx;
        let entry = emu.regs().rdx;
        log_red!(emu, "kernel32!{api} snap:0x{:x} entry:0x{:x}", snap, entry);
        if entry != 0 {
            // PROCESSENTRY32W.dwSize at offset 0
            emu.maps.write_dword(entry, 556); // sizeof(PROCESSENTRY32W)
            // th32ProcessID at offset 8
            emu.maps.write_dword(entry + 8, 4); // System process
            // cntThreads at offset 20
            emu.maps.write_dword(entry + 20, 1);
            // th32ParentProcessID at offset 24
            emu.maps.write_dword(entry + 24, 0);
            // szExeFile at offset 44 (wide string)
            strings::write_string(emu, entry + 44, "System", wide);
        }
    } else {
        let handle = emu.regs().rcx;
        let lppe = emu.regs().rdx;
        log_red!(emu, "kernel32!{api} hndl:0x{:x} entry:0x{:x}", handle, lppe);
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
        emu.maps.write_dword(lppe + 4, 0);
        emu.maps.write_dword(lppe + 8, 4);
        emu.maps.write_qword(lppe + 16, 0);
        emu.maps.write_dword(lppe + 24, 0);
        emu.maps.write_dword(lppe + 28, 2);
        emu.maps.write_dword(lppe + 32, 0);
        emu.maps.write_dword(lppe + 36, 8);
        emu.maps.write_dword(lppe + 40, 0);
        strings::write_string(emu, lppe + 44, "smss.exe", wide);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
