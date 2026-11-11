use crate::emu;

pub fn PathAppendA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!PathAppendA '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn PathAppendW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let more_ptr = emu.regs().rdx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let more = if more_ptr != 0 {
        emu.maps.read_wide_string(more_ptr)
    } else {
        String::new()
    };
    let result = if path.is_empty() {
        more.clone()
    } else if more.is_empty() {
        path.clone()
    } else {
        let sep = if path.ends_with('\\') || path.ends_with('/') {
            ""
        } else {
            "\\"
        };
        format!("{}{}{}", path, sep, more)
    };
    if path_ptr != 0 {
        emu.maps.write_wide_string(path_ptr, &result);
    }
    log_red!(
        emu,
        "kernelbase!PathAppendW '{}' + '{}' => '{}'",
        path,
        more,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
