use crate::emu;

pub fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let table_ptr = emu.regs().rcx;
    if table_ptr != 0 {
        emu.maps.write_qword(table_ptr, 0);
        emu.maps.write_qword(table_ptr + 8, 0);
        emu.maps.write_qword(table_ptr + 16, 0);
    }
    log_red!(emu, "msvcrt!_initialize_onexit_table tbl:0x{:x}", table_ptr);
    emu.regs_mut().rax = 0;
}
