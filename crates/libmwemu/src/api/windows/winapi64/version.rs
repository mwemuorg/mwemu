use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "GetFileVersionInfoSizeA" | "GetFileVersionInfoSizeW" => GetFileVersionInfoSize(emu),
        "GetFileVersionInfoA" | "GetFileVersionInfoW" => GetFileVersionInfo(emu),
        "VerQueryValueA" | "VerQueryValueW" => VerQueryValue(emu),
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

fn GetFileVersionInfoSize(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let handle_ptr = emu.regs().rdx;

    let filename = emu.maps.read_string(filename_ptr);
    log_red!(emu, "version!GetFileVersionInfoSize `{}`", filename);

    if handle_ptr != 0 {
        emu.maps.write_dword(handle_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}

fn GetFileVersionInfo(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let _handle = emu.regs().rdx;
    let _len = emu.regs().r8;
    let _data = emu.regs().r9;

    let filename = emu.maps.read_string(filename_ptr);
    log_red!(emu, "version!GetFileVersionInfo `{}`", filename);

    emu.regs_mut().rax = 0;
}

fn VerQueryValue(emu: &mut emu::Emu) {
    let _block = emu.regs().rcx;
    let subblock_ptr = emu.regs().rdx;
    let _buffer = emu.regs().r8;
    let _len = emu.regs().r9;

    let subblock = emu.maps.read_string(subblock_ptr);
    log_red!(emu, "version!VerQueryValue `{}`", subblock);

    emu.regs_mut().rax = 0;
}
