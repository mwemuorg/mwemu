use crate::emu;

pub fn WsaStartup_api(emu: &mut emu::Emu) {
    let version = emu.regs().rcx as u32;
    let wsa_data_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WsaStartup ver:0x{:x} data:0x{:x}",
        version,
        wsa_data_ptr
    );
    if wsa_data_ptr != 0 {
        emu.maps.write_word(wsa_data_ptr, version as u16);
        emu.maps.write_word(wsa_data_ptr + 2, version as u16);
    }
    emu.regs_mut().rax = 0;
}
