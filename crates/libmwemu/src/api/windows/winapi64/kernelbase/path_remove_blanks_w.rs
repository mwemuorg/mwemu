use crate::emu;

pub fn PathRemoveBlanksW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    if path_ptr != 0 {
        let path = emu.maps.read_wide_string(path_ptr);
        let trimmed = path.trim().to_string();
        emu.maps.write_wide_string(path_ptr, &trimmed);
        log_red!(
            emu,
            "kernelbase!PathRemoveBlanksW '{}' => '{}'",
            path,
            trimmed
        );
    }
}
