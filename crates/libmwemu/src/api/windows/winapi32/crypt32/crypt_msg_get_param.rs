use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptMsgGetParam(emu: &mut emu::Emu) {
    let hmsg = WinApiAbi::arg(emu, 0);
    let param_type = WinApiAbi::arg(emu, 1) as u32;
    let index = WinApiAbi::arg(emu, 2) as u32;
    let pv_data = WinApiAbi::arg(emu, 3);
    let pcb_data = WinApiAbi::arg(emu, 4);
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
