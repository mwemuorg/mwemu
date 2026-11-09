use crate::emu;

pub fn CommandLineToArgvW(emu: &mut emu::Emu) {
    let cmd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let argc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cmd = emu.maps.read_wide_string(cmd_ptr);
    if argc_ptr != 0 {
        emu.maps.write_dword(argc_ptr, 1);
    }
    let argv_alloc = emu
        .maps
        .alloc(256)
        .expect("shell32!CommandLineToArgvW alloc");
    emu.maps.write_dword(argv_alloc, (argv_alloc + 4) as u32);
    emu.maps.write_wide_string(argv_alloc + 4, &cmd);
    log_red!(
        emu,
        "shell32!CommandLineToArgvW cmd:'{}' => 0x{:x}",
        cmd,
        argv_alloc
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = argv_alloc;
}
