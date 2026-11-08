use crate::emu;

pub fn CommandLineToArgvW(emu: &mut emu::Emu) {
    let cmd_line_ptr = emu.regs().rcx;
    let argc_ptr = emu.regs().rdx;

    let cmd = if cmd_line_ptr != 0 {
        emu.maps.read_wide_string(cmd_line_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "shell32!CommandLineToArgvW cmd: '{}'", cmd);

    // Allocate space for the argv array + string data
    let alloc_size = 1024u64;
    let addr = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            emu.regs_mut().rax = 42;
            return;
        }
    };
    emu.maps.create_map(
        "cmdline_argv",
        addr,
        alloc_size,
        crate::maps::mem64::Permission::READ_WRITE,
    );

    // Write the command string as a single argv[0] wide string
    let str_offset = 64u64;
    emu.maps.write_wide_string(addr + str_offset, &cmd);
    // argv[0] pointer
    emu.maps.write_qword(addr, addr + str_offset);

    if argc_ptr != 0 {
        emu.maps.write_dword(argc_ptr, 1); // 1 argument
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
