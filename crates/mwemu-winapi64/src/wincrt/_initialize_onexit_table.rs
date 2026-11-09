use crate::emu;

pub(super) fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;

    log_red!(emu, "wincrt!_initialize_onexit_table 0x{:x}", table);

    if table != 0 {
        emu.maps.write_qword(table, 0); // _first
        emu.maps.write_qword(table + 8, 0); // _last
        emu.maps.write_qword(table + 16, 0); // _end
    }

    emu.regs_mut().rax = 0;
}
