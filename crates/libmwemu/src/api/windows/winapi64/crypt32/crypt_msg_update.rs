use crate::emu;
use crate::winapi::helper;

pub fn CryptMsgUpdate(emu: &mut emu::Emu) {
    let hmsg = emu.regs().rcx;
    let pb_data = emu.regs().rdx;
    let cb_data = emu.regs().r8;
    let f_final = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CryptMsgUpdate msg:0x{:x} data:0x{:x} len:{} final:{}",
        hmsg,
        pb_data,
        cb_data,
        f_final
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    if hmsg != 0 && pb_data != 0 && cb_data > 0 {
        let mut buf = vec![0u8; cb_data as usize];
        for i in 0..cb_data {
            buf[i as usize] = emu.maps.read_byte(pb_data + i).unwrap_or(0);
        }
        helper::handler_put_bytes(hmsg, &buf);
    }
    emu.regs_mut().rax = 1;
}
