use crate::emu;

pub fn Module32First(emu: &mut emu::Emu) {
    let h_snapshot = emu.regs().rcx;
    let p_entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Module32First snap:0x{:x} entry:0x{:x}",
        h_snapshot,
        p_entry
    );

    if p_entry == 0 {
        emu.set_last_error(87); // ERROR_INVALID_PARAMETER
        emu.regs_mut().rax = 0;
        return;
    }

    // Fill MODULEENTRY32W structure (1080 bytes on 64-bit)
    let dw_size = emu.maps.read_dword(p_entry).unwrap_or(0);
    if dw_size < 568 {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
        return;
    }

    // dwSize already set by caller
    // th32ModuleID
    emu.maps.write_dword(p_entry + 4, 1);
    // th32ProcessID
    emu.maps.write_dword(p_entry + 8, 0x123);
    // GlblcntUsage
    emu.maps.write_dword(p_entry + 12, 0xFFFF);
    // ProccntUsage
    emu.maps.write_dword(p_entry + 16, 0xFFFF);
    // modBaseAddr (pointer, 8 bytes on 64-bit)
    emu.maps
        .write_qword(p_entry + 24, emu.regs().rip & 0xFFFF_FFFF_FFFF_0000);
    // modBaseSize
    emu.maps.write_dword(p_entry + 32, 0x10000);
    // hModule (pointer)
    emu.maps
        .write_qword(p_entry + 40, emu.regs().rip & 0xFFFF_FFFF_FFFF_0000);
    // szModule at offset 48 (wide string, MAX_MODULE_NAME32+1 = 256 wchars)
    emu.maps.write_wide_string(p_entry + 48, "main.exe");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}

pub fn Module32FirstW(emu: &mut emu::Emu) {
    let snapshot = emu.regs().rcx;
    let module_entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Module32FirstW snapshot:0x{:x} entry:0x{:x}",
        snapshot,
        module_entry
    );

    if module_entry != 0 {
        // MODULEENTRY32W.dwSize at offset 0
        emu.maps.write_dword(module_entry, 0x224);
        // th32ModuleID at offset 4
        emu.maps.write_dword(module_entry + 4, 1);
        // th32ProcessID at offset 8
        emu.maps.write_dword(module_entry + 8, 0x1234);
        // modBaseAddr at offset 0x18
        let base = emu.maps.get_base().unwrap_or(0x140000000);
        emu.maps.write_qword(module_entry + 0x18, base);
        // modBaseSize at offset 0x20
        emu.maps.write_dword(module_entry + 0x20, 0x10000);
        // szModule (wide) at offset 0x28 — write "program.exe"
        let name = "program.exe";
        for (i, b) in name.bytes().enumerate() {
            emu.maps
                .write_word(module_entry + 0x28 + (i as u64) * 2, b as u16);
        }
        emu.maps
            .write_word(module_entry + 0x28 + (name.len() as u64) * 2, 0);
    }

    emu.regs_mut().rax = 1; // TRUE
}
