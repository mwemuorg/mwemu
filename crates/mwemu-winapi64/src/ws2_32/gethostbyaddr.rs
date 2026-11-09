use crate::emu;

pub fn gethostbyaddr(emu: &mut emu::Emu) {
    let addr_ptr = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    let typ = emu.regs().r8 as u32;
    log_red!(
        emu,
        "ws2_32!gethostbyaddr addr:0x{:x} len:{} type:{}",
        addr_ptr,
        len,
        typ
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
