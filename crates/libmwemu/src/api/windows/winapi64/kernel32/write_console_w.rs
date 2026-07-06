use crate::emu;

pub fn WriteConsoleW(emu: &mut emu::Emu) {
    let h_console_output = emu.regs().rcx;
    let lp_buffer = emu.regs().rdx as usize;
    let n_number_of_chars_to_write = emu.regs().r8 as u32;
    let lp_number_of_chars_written = emu.regs().r9 as usize;
    let lp_reserved = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!WriteConsoleW cannot read_qword lp_reserved");

    // Read the UTF-16 buffer
    let mut wide_chars = Vec::new();
    for i in 0..n_number_of_chars_to_write {
        let wchar = emu
            .maps
            .read_word(lp_buffer as u64 + (i * 2) as u64)
            .unwrap();
        wide_chars.push(wchar);
    }

    // Convert UTF-16 to String for logging
    let s1 = String::from_utf16_lossy(&wide_chars);

    log_red!(
        emu,
        "** {} kernel32!WriteConsoleW h_console_output = {:x} lp_buffer = {:x} n_number_of_chars_to_write = {:x} lp_number_of_chars_written = {:x} lp_reserved = {:x} s1 = {}",
        emu.pos,
        h_console_output,
        lp_buffer,
        n_number_of_chars_to_write,
        lp_number_of_chars_written,
        lp_reserved,
        s1
    );

    // Write back the number of characters written
    emu.maps.write_dword(
        lp_number_of_chars_written as u64,
        n_number_of_chars_to_write,
    );
    emu.regs_mut().rax = 1;
}
