use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "BCryptOpenAlgorithmProvider" => BCryptOpenAlgorithmProvider(emu),
        "BCryptGenRandom" => BCryptGenRandom(emu),
        "BCryptCloseAlgorithmProvider" => BCryptCloseAlgorithmProvider(emu),
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

pub fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let ph_algorithm = emu.regs().rcx;
    let psz_alg_id = emu.regs().rdx;
    let flags = emu.regs().r9;

    let algo_name = emu.maps.read_wide_string(psz_alg_id);

    log_red!(
        emu,
        "bcrypt!BCryptOpenAlgorithmProvider algo: {} flags: 0x{:x}",
        algo_name,
        flags
    );

    let handle = helper::handler_create(&format!("bcrypt://{}", algo_name));
    if ph_algorithm != 0 {
        emu.maps.write_qword(ph_algorithm, handle);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptGenRandom(emu: &mut emu::Emu) {
    let pb_buffer = emu.regs().rdx;
    let cb_buffer = emu.regs().r8;
    let flags = emu.regs().r9;

    log_red!(
        emu,
        "bcrypt!BCryptGenRandom buf: 0x{:x} len: {} flags: 0x{:x}",
        pb_buffer,
        cb_buffer,
        flags
    );

    for i in 0..cb_buffer {
        let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
        emu.maps.write_byte(pb_buffer + i, byte);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x} flags: 0x{:x}",
        h_algorithm,
        flags
    );

    helper::handler_close(h_algorithm);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
