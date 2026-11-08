use crate::emu;

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let lp_class = emu.regs().rdx;
    let lpcch_class = emu.regs().r8;
    let _reserved = emu.regs().r9;
    let lpc_sub_keys = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let lpcb_max_sub_key_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let lpcb_max_class_len = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let lpc_values = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let lpcb_max_value_name_len = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let lpcb_max_value_len = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);
    let lpcb_security_desc = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0);
    let lpft_last_write = emu.maps.read_qword(emu.regs().rsp + 0x60).unwrap_or(0);

    log_red!(emu, "kernelbase!RegQueryInfoKeyW hkey:0x{:x}", hkey);

    if lpcch_class != 0 {
        emu.maps.write_dword(lpcch_class, 0);
    }
    if lpc_sub_keys != 0 {
        emu.maps.write_dword(lpc_sub_keys, 0);
    }
    if lpcb_max_sub_key_len != 0 {
        emu.maps.write_dword(lpcb_max_sub_key_len, 0);
    }
    if lpcb_max_class_len != 0 {
        emu.maps.write_dword(lpcb_max_class_len, 0);
    }
    if lpc_values != 0 {
        emu.maps.write_dword(lpc_values, 0);
    }
    if lpcb_max_value_name_len != 0 {
        emu.maps.write_dword(lpcb_max_value_name_len, 0);
    }
    if lpcb_max_value_len != 0 {
        emu.maps.write_dword(lpcb_max_value_len, 0);
    }
    if lpcb_security_desc != 0 {
        emu.maps.write_dword(lpcb_security_desc, 0);
    }
    if lpft_last_write != 0 {
        emu.maps.write_qword(lpft_last_write, 0);
    }
    emu.regs_mut().rax = 0;
}
