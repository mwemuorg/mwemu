use crate::emu;

pub fn WSASetServiceA(emu: &mut emu::Emu) {
    let query_set_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WSASetServiceA op:{} flags:0x{:x}",
        operation,
        flags
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
