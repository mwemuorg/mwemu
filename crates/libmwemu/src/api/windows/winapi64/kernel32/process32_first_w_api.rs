use crate::emu;

pub fn Process32FirstW_api(emu: &mut emu::Emu) {
    let snap = emu.regs().rcx;
    let entry = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!Process32FirstW snap:0x{:x} entry:0x{:x}",
        snap,
        entry
    );
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
        emu.maps.write_wide_string(entry + 44, "System");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
