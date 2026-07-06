use crate::emu;
use crate::windows::peb::peb32;

pub fn GetProcAddress(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetProcAddress cannot read the handle") as u64;
    let func_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetProcAddress cannot read the func name") as u64;
    let func = emu.maps.read_string(func_ptr).to_lowercase();

    //log::trace!("looking for '{}'", func);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    //peb32::show_linked_modules(emu);

    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }
                let ordinal = flink.get_function_ordinal(emu, i);

                //log::trace!("func name {}!{}", flink.mod_name, ordinal.func_name);

                if ordinal.func_name.to_lowercase() == func {
                    emu.regs_mut().rax = ordinal.func_va;
                    log_red!(
                        emu,
                        "kernel32!GetProcAddress  `{}!{}` =0x{:x}",
                        flink.mod_name,
                        ordinal.func_name,
                        emu.regs().get_eax() as u32
                    );
                    return;
                }
            }
        }

        flink.next(emu);
        if flink.get_ptr() == first_flink {
            break;
        }
    }
    emu.regs_mut().rax = 0;
    log::warn!("kernel32!GetProcAddress error searching {}", func);
}
