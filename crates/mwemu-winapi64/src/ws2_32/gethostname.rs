use crate::emu;

pub fn gethostname(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let namelen = emu.regs().rdx as u32;
    log_red!(
        emu,
        "ws2_32!gethostname buf:0x{:x} len:{}",
        name_ptr,
        namelen
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if name_ptr != 0 && namelen > 7 {
        emu.maps.write_string(name_ptr, "DESKTOP");
    }
    emu.regs_mut().rax = 0;
}
