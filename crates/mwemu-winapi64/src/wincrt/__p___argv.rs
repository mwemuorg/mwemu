use super::allocate_memory;
use crate::emu;

pub(super) fn __p___argv(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!__p___argv");

    // First, allocate space for argv array (pointer array)
    // We'll allocate space for 2 pointers - one for program name and null terminator
    let argv_array_addr = match allocate_memory(emu, 16) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argv cannot allocate argv array");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    // Allocate space for program name string (using a dummy name)
    let prog_name = "program.exe\0";
    let prog_name_addr = match allocate_memory(emu, prog_name.len() as u64) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argv cannot allocate program name");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    // Write program name string
    emu.maps.write_string(prog_name_addr, prog_name);

    // Write argv array:
    // argv[0] = pointer to program name
    emu.maps.write_qword(argv_array_addr, prog_name_addr);
    // argv[1] = null terminator
    emu.maps.write_qword(argv_array_addr + 8, 0);

    // Allocate space for pointer to argv array
    let p_argv_addr = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argv cannot allocate p_argv");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    // Write pointer to argv array
    emu.maps.write_qword(p_argv_addr, argv_array_addr);

    // Return pointer to argv
    emu.regs_mut().rax = p_argv_addr;
}
