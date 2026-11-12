use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptSignMessage(emu: &mut emu::Emu) {
    let p_sign_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let f_detached_signature = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let c_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let rgb_pb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let rgcb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptSignMessage 0x{:x}", p_sign_para);
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
