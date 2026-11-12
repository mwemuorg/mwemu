use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let table_ptr = WinApiAbi::arg(emu, 0);
    if table_ptr != 0 {
        emu.maps.write_dword(table_ptr, 0); // _first
        emu.maps.write_dword(table_ptr + 4, 0); // _last
        emu.maps.write_dword(table_ptr + 8, 0); // _end
    }
    log_red!(emu, "msvcrt!_initialize_onexit_table tbl:0x{:x}", table_ptr);
    emu.regs_mut().rax = 0;
}
