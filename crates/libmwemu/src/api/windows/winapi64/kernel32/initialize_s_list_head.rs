use crate::emu;

pub fn InitializeSListHead(emu: &mut emu::Emu) {
    let list_head = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!InitializeSListHead listHead:0x{:x}",
        list_head
    );

    if list_head != 0 {
        emu.maps.write_qword(list_head, 0);
        emu.maps.write_qword(list_head + 8, 0);
    }
}
