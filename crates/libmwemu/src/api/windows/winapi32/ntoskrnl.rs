use crate::emu;
use crate::serialization;
//use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::windows::constants;
use crate::windows::structures::UnicodeString;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "RtlInitUnicodeString" => RtlInitUnicodeString(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn RtlInitUnicodeString(emu: &mut emu::Emu) {
    let dst_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("ntoskrnl!RtlInitUnicodeString: error reading arg1 (dst_ptr)") as u64;
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntoskrnl!RtlInitUnicodeString: error reading optional arg2 (src_ptr)")
        as u64;

    if !emu.maps.is_mapped(dst_ptr) {
        log_red!(
            emu,
            "ntoskrnl!RtlInitUnicodeString worng destination pointer 0x{:x}",
            dst_ptr
        );
        panic!();
    }

    let mut s = "".to_string();
    let ustr;

    if src_ptr > 0 && emu.maps.is_mapped(src_ptr) {
        ustr = UnicodeString::load(src_ptr, &emu.maps);
        if emu.maps.is_mapped(ustr.buffer as u64) {
            s = emu.maps.read_wide_string(ustr.buffer as u64);
        } else {
            log_red!(
                emu,
                "ntoskrnl!RtlInitUnicodeString ustr.buffer is not ok: 0x{:x}",
                ustr.buffer
            );
        }
    } else {
        ustr = UnicodeString::new();
    }

    ustr.save(dst_ptr, &mut emu.maps);

    log_red!(
        emu,
        "ntoskrnl!RtlInitUnicodeString dst: 0x{:x} str:'{}' src: 0x{:x}",
        dst_ptr,
        s,
        src_ptr
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
