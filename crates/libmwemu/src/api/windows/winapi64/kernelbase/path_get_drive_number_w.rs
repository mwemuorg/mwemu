use crate::emu;

pub fn PathGetDriveNumberW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let result = if path.len() >= 2 && path.as_bytes()[1] == b':' {
        let ch = path.as_bytes()[0].to_ascii_uppercase();
        if ch.is_ascii_uppercase() {
            (ch - b'A') as u64
        } else {
            u64::MAX
        }
    } else {
        u64::MAX
    };
    log_red!(
        emu,
        "kernelbase!PathGetDriveNumberW '{}' => {}",
        path,
        result as i64
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
