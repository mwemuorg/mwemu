use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptDecodeObject(emu: &mut emu::Emu) {
    let dw_cert_encoding = WinApiAbi::arg(emu, 0);
    let lpsz_struct_type = WinApiAbi::arg(emu, 1);
    let pb_encoded = WinApiAbi::arg(emu, 2);
    let cb_encoded = WinApiAbi::arg(emu, 3);
    let dw_flags = WinApiAbi::arg(emu, 4);
    let pv_struct_info = WinApiAbi::arg(emu, 5);
    let pcb_struct_info = WinApiAbi::arg(emu, 6);
    log_red!(emu, "crypt32!CryptDecodeObject 0x{:x}", dw_cert_encoding);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
