use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = arg(emu, 0);
    let struct_type = arg(emu, 1);
    let _encoded = arg(emu, 2);
    let cb_encoded = arg(emu, 3);
    let flags = arg(emu, 4);
    let _decode_para = arg(emu, 5);
    let _pv_struct_info = arg(emu, 6);
    let pcb_struct_info = arg(emu, 7) as u64;

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info, 0);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
