use crate::emu;

pub fn CryptMsgGetParam(emu: &mut emu::Emu) {
    let hmsg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let pv_data = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pcb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptMsgGetParam msg:0x{:x} type:{} idx:{}",
        hmsg,
        param_type,
        index
    );
    if pcb_data != 0 {
        emu.maps.write_dword(pcb_data, 0);
    }
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
