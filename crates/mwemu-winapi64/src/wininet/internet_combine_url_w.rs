use crate::emu;

pub fn InternetCombineUrlW(emu: &mut emu::Emu) {
    let base_ptr = emu.regs().rcx;
    let relative_ptr = emu.regs().rdx;
    let combined_ptr = emu.regs().r8;
    let combined_len_ptr = emu.regs().r9;
    let base = if base_ptr != 0 {
        emu.maps.read_wide_string(base_ptr)
    } else {
        String::new()
    };
    let relative = if relative_ptr != 0 {
        emu.maps.read_wide_string(relative_ptr)
    } else {
        String::new()
    };
    let combined = if relative.starts_with("http://") || relative.starts_with("https://") {
        relative.clone()
    } else {
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            relative.trim_start_matches('/')
        )
    };
    log_red!(
        emu,
        "wininet!InternetCombineUrlW base:'{}' rel:'{}' => '{}'",
        base,
        relative,
        combined
    );
    if combined_ptr != 0 {
        emu.maps.write_wide_string(combined_ptr, &combined);
    }
    if combined_len_ptr != 0 {
        emu.maps
            .write_dword(combined_len_ptr, combined.len() as u32);
    }
    emu.regs_mut().rax = 1;
}
