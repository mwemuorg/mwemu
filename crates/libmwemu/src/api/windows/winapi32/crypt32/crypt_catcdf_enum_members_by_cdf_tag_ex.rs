use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATCDFEnumMembersByCDFTagEx(emu: &mut emu::Emu) {
    let p_cdf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pwsz_prev_tag = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pp_member = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let f_continue_on_error = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumMembersByCDFTagEx 0x{:x}",
        p_cdf
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
