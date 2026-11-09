use crate::emu;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let ft_ptr = emu.regs().rcx;
    if ft_ptr != 0 {
        emu.maps.write_dword(ft_ptr, 0xd53e8000);
        emu.maps.write_dword(ft_ptr + 4, 0x019db1de);
    }
    log_red!(emu, "kernelbase!GetSystemTimeAsFileTime");
}
