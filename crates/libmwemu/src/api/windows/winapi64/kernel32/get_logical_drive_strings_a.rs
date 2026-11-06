use crate::emu;

pub fn GetLogicalDriveStringsA(emu: &mut emu::Emu) {
    let buf_len = emu.regs().rcx as u32;
    let buff_ptr = emu.regs().rdx;

    log_red!(emu, "kernel32!GetLogicalDriveStringsA len:{}", buf_len);
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    // Returns "C:\\\0\0" — each drive string null-terminated, extra null at end
    let drives = b"C:\\\0\0";
    let needed = (drives.len() - 1) as u32; // exclude final null from count

    if buff_ptr != 0 && buf_len >= drives.len() as u32 {
        for (i, &b) in drives.iter().enumerate() {
    emu.maps.write_byte(buff_ptr + i as u64, b);
        }
    }

    emu.regs_mut().rax = needed as u64;
}
