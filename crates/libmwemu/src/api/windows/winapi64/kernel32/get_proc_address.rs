use crate::emu;
use crate::windows::peb::peb64;

pub fn GetProcAddress(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let func_ptr = emu.regs().rdx;
    let func = emu.maps.read_string(func_ptr).to_lowercase();
    /*
    if func == "zwcopyfilechunk" {
        emu.regs_mut().rax = 0x7ff7e0001337;
        log_red!(
            emu,
            "kernel32!GetProcAddress  `{}!{}` =0x{:x}",
            "kernel32",
            "zwcopyfilechunk",
            emu.regs().rax
        );
        return;
    }*/

    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }
                let ordinal = flink.get_function_ordinal(emu, i);

                // log::trace!("func name {}!{}", flink.mod_name, ordinal.func_name);

                if ordinal.func_name.to_lowercase() == func {
                    emu.regs_mut().rax = ordinal.func_va;
                    if emu.cfg.verbose >= 1 {
                        log_red!(
                            emu,
                            "kernel32!GetProcAddress  `{}!{}` =0x{:x}",
                            flink.mod_name,
                            ordinal.func_name,
                            emu.regs().rax
                        );
                    }
                    return;
                }
            }
        }

        if !flink.next(emu) || flink.get_ptr() == first_flink {
            break;
        }
    }
    emu.regs_mut().rax = 0;
    log::warn!("kernel32!GetProcAddress error searching {}", func);
}
