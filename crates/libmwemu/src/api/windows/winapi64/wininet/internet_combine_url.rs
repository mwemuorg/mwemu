use crate::emu;
use crate::winapi::common::strings;

pub fn InternetCombineUrlA(emu: &mut emu::Emu) {
    internet_combine_url(emu, "InternetCombineUrlA", false);
}

fn internet_combine_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let base_ptr = emu.regs().rcx;
    let relative_ptr = emu.regs().rdx;
    let combined_ptr = emu.regs().r8;
    let combined_len_ptr = emu.regs().r9;
    let base = if base_ptr != 0 {
        strings::read_text(emu, base_ptr, wide)
    } else {
        String::new()
    };
    let relative = if relative_ptr != 0 {
        strings::read_text(emu, relative_ptr, wide)
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
        "wininet!{api} base:'{}' rel:'{}' => '{}'",
        base,
        relative,
        combined
    );
    if combined_ptr != 0 {
        strings::write_string(emu, combined_ptr, &combined, wide);
    }
    if combined_len_ptr != 0 {
        emu.maps
            .write_dword(combined_len_ptr, combined.len() as u32);
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetCombineUrlW(emu: &mut emu::Emu) {
    internet_combine_url(emu, "InternetCombineUrlW", true);
}
