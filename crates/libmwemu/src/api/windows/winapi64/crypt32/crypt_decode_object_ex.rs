use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = emu.regs().rcx;
    let struct_type = emu.regs().rdx;
    let encoded = emu.regs().r8;
    let cb_encoded = emu.regs().r9;
    let flags = WinApiAbi::arg(emu, 4);
    let decode_para = WinApiAbi::arg(emu, 5);
    let pv_struct_info = WinApiAbi::arg(emu, 6);
    let pcb_struct_info = WinApiAbi::arg(emu, 7);

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} encoded: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        encoded,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info, 0);
    }

    emu.regs_mut().rax = 1;
}
