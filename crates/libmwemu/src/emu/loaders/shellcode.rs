//! Raw shellcode: machine code with no executable header.

use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::mem64::Permission;
use crate::windows::peb::{peb32, peb64};

use super::format::{FormatLoader, LoadInput, apply_entry_point_override};

/// Loader for raw machine code.
pub(crate) struct Shellcode;

impl FormatLoader for Shellcode {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        log::trace!("shellcode detected.");

        // TODO: the register/flag reset should follow the caller's request
        // instead of being hardcoded off.
        emu.init_win32(false, false)?;

        // The shellcode runs as a thread of a normal Windows process, so the
        // process image is mapped first and the shellcode goes on top of it.
        let exe_name = emu.cfg.exe_name.clone();
        let exe_path = emu.cfg.get_maps_folder(&exe_name);
        if emu.cfg.is_x64() {
            match emu.load_pe64(&exe_path, false, 0) {
                Ok(image) => peb64::update_ldr_entry_base(&exe_name, image.base, emu),
                Err(error) => log::warn!("cannot load shellcode process image: {error}"),
            }
        } else {
            match emu.load_pe32(&exe_path, false, 0) {
                Ok(image) => peb32::update_ldr_entry_base(&exe_name, image.base, emu),
                Err(error) => log::warn!("cannot load shellcode process image: {error}"),
            }
        }

        let code_sz = input.bytes.len().max(1) as u64;
        let code_map = match emu.maps.create_map(
            "code",
            emu.cfg.code_base_addr,
            code_sz,
            Permission::READ_WRITE_EXECUTE,
        ) {
            Ok(m) => m,
            Err(_) => {
                log::error!("cannot create code map");
                return Err(MwemuError::new("cannot create code map"));
            }
        };
        if !code_map.load(input.path) {
            log::trace!("shellcode not found, select the file with -f");
            return Err(MwemuError::new(&format!(
                "cannot load shellcode from '{}'",
                input.path
            )));
        }
        let code = emu.maps.get_mem_mut("code");
        code.extend(0xffff); // this could overlap an existing map

        apply_entry_point_override(emu);
        Ok(())
    }
}
