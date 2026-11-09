use crate::emu;

pub fn CryptMsgGetParam(emu: &mut emu::Emu) {
    let hmsg = emu.regs().rcx;
    let param_type = emu.regs().rdx;
    let index = emu.regs().r8;
    let pv_data = emu.regs().r9;
    let pcb_data = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
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
    emu.regs_mut().rax = 1;
}
