use crate::emu;

pub fn LocalFileTimeToFileTime(emu: &mut emu::Emu) {
    let local_ft_ptr = emu.regs().rcx;
    let ft_ptr = emu.regs().rdx;

    let low = emu.maps.read_dword(local_ft_ptr).unwrap_or(0);
    let high = emu.maps.read_dword(local_ft_ptr + 4).unwrap_or(0);

    if ft_ptr != 0 {
        emu.maps.write_dword(ft_ptr, low);
        emu.maps.write_dword(ft_ptr + 4, high);
    }

    log_red!(
        emu,
        "kernel32!LocalFileTimeToFileTime 0x{:x} -> 0x{:x}",
        local_ft_ptr,
        ft_ptr
    );
    emu.regs_mut().rax = 1;
}
