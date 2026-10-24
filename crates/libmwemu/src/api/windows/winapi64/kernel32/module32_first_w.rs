use crate::emu;

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
