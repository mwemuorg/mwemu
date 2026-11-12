use crate::emu;

pub(super) fn LoadStringA(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringA", false);
}

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringW", true);
}

fn load_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hinst = emu.regs().rcx;
    let id = emu.regs().rdx;
    let buf = emu.regs().r8;
    let buf_max = emu.regs().r9 as usize;
    log_red!(emu, "user32!{api} id:{} buf:0x{:x}", id, buf);
    if wide {
        if buf != 0 && buf_max > 0 {
            emu.maps.write_word(buf, 0);
        }
    } else {
        if buf != 0 && buf_max > 0 {
            emu.maps.write_byte(buf, 0);
        }
    }

    emu.regs_mut().rax = 0;
}
