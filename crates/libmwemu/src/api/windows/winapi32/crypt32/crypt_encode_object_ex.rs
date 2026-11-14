use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptEncodeObjectEx(emu: &mut emu::Emu) {
    let encoding = WinApiAbi::arg(emu, 0);
    let struct_type = WinApiAbi::arg(emu, 1);
    let struct_info = WinApiAbi::arg(emu, 2);
    let flags = WinApiAbi::arg(emu, 3);
    let p_encode_para = WinApiAbi::arg(emu, 4);
    let pv_encoded = WinApiAbi::arg(emu, 5);
    let pcb_encoded = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
        encoding,
        struct_type,
        flags
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
}
