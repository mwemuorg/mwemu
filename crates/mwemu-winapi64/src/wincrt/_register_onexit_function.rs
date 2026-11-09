use crate::emu;

pub(super) fn _register_onexit_function(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let callback = emu.regs().rdx;

    log_red!(
        emu,
        "wincrt!_register_onexit_function table:0x{:x} callback:0x{:x}",
        table,
        callback
    );

    if table != 0 {
        let last = emu.maps.read_qword(table + 8).unwrap_or(0);
        let end = emu.maps.read_qword(table + 16).unwrap_or(0);
        if last < end && last != 0 {
            emu.maps.write_qword(last, callback);
            emu.maps.write_qword(table + 8, last + 8);
        }
    }

    emu.regs_mut().rax = 0;
}
