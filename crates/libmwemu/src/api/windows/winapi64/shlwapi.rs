use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;
//use crate::windows::constants;
//use crate::winapi::helper;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PathIsContentTypeW" => PathIsContentTypeW(emu),
        "PathFindSuffixArrayA" => PathFindSuffixArrayA(emu),
        "PathFileExistsA" => PathFileExistsA(emu),
        "PathFileExistsW" => PathFileExistsW(emu),

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

pub fn PathIsContentTypeW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let content_type_ptr = emu.regs().rdx;

    let mut path = String::new();
    let mut content_type = String::new();

    if path_ptr > 0 {
        path = emu.maps.read_wide_string(path_ptr);
    }
    if content_type_ptr > 0 {
        content_type = emu.maps.read_wide_string(content_type_ptr);
    }

    log_red!(
        emu,
        "shlwapi!PathIsContentTypeW path: {} content-type: {}",
        path,
        content_type
    );

    emu.regs_mut().rax = 1;
}

pub fn PathFindSuffixArrayA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let suffixes_ptr = emu.regs().rdx;

    let mut path = String::new();
    let mut suffixes = String::new();

    if path_ptr > 0 {
        path = emu.maps.read_string(path_ptr);
    }
    if suffixes_ptr > 0 {
        suffixes = emu.maps.read_string(suffixes_ptr);
    }

    log_red!(
        emu,
        "shlwapi!PathFindSuffixArrayA path: {} suffixes: {}",
        path,
        suffixes
    );

    emu.regs_mut().rax = emu.regs().rdx;
}

fn PathFileExistsA(emu: &mut emu::Emu) {
    let ptr_path = emu.regs().rcx;
    let path = emu.maps.read_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsA path: {}", path);

    emu.regs_mut().rax = 1; // True
}

fn PathFileExistsW(emu: &mut emu::Emu) {
    let ptr_path = emu.regs().rcx;
    let path = emu.maps.read_wide_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsW path: {}", path);

    emu.regs_mut().rax = 1; // True
}
