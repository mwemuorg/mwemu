use crate::emu;

pub fn GetClassNameW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    let class_name = "mwemu_class";
    if buf != 0 && max_count > 0 {
        emu.maps.write_wide_string(buf, class_name);
    }
    log_red!(
        emu,
        "user32!GetClassNameW hwnd:0x{:x} => '{}'",
        hwnd,
        class_name
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = class_name.len() as u64;
}
