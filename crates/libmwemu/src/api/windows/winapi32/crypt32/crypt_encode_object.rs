use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptEncodeObject(emu: &mut emu::Emu) {
    let dw_cert_encoding = WinApiAbi::arg(emu, 0);
    let lpsz_struct_type = WinApiAbi::arg(emu, 1);
    let pv_struct_info = WinApiAbi::arg(emu, 2);
    let pb_encoded = WinApiAbi::arg(emu, 3);
    let pcb_encoded = WinApiAbi::arg(emu, 4);
    let dw_flags = WinApiAbi::arg(emu, 5);
    log_red!(emu, "crypt32!CryptEncodeObject 0x{:x}", dw_cert_encoding);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
