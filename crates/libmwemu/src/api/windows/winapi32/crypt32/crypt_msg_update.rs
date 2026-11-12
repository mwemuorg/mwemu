use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptMsgUpdate(emu: &mut emu::Emu) {
    let hmsg = WinApiAbi::arg(emu, 0);
    let pb_data = WinApiAbi::arg(emu, 1);
    let cb_data = WinApiAbi::arg(emu, 2) as u32;
    let f_final = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "crypt32!CryptMsgUpdate msg:0x{:x} data:0x{:x} len:{} final:{}",
        hmsg,
        pb_data,
        cb_data,
        f_final
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    if hmsg != 0 && pb_data != 0 && cb_data > 0 {
        let mut buf = vec![0u8; cb_data as usize];
        for i in 0..cb_data as u64 {
            buf[i as usize] = emu.maps.read_byte(pb_data + i).unwrap_or(0);
        }
        helper::handler_put_bytes(hmsg, &buf);
    }
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
