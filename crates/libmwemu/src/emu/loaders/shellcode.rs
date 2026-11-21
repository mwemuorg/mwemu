//! Raw machine code with no executable header.

use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::mem64::Permission;
use crate::windows::peb::{peb32, peb64};

use super::format::{FormatLoader, LoadInput, apply_entry_point_override};

// File-backed raw code has historically had this much zero-filled space after the input.
const RAW_CODE_PADDING: u64 = 0xffff;

pub(crate) struct Shellcode;

impl FormatLoader for Shellcode {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        log::trace!("shellcode detected.");
        if emu.os.is_windows() {
            init_windows_process(emu)?;
        } else {
            emu.init_cpu();
        }

        let base = emu.cfg.code_base_addr;
        let size = input.bytes.len() as u64 + RAW_CODE_PADDING;
        let code = emu
            .maps
            .create_map("code", base, size, Permission::READ_WRITE_EXECUTE)
            .map_err(|reason| {
                MwemuError::new(&format!("cannot map raw code at {base:#x}: {reason}"))
            })?;
        code.write_bytes(base, input.bytes);
        emu.set_pc(base);
        apply_entry_point_override(emu);
        Ok(())
    }
}

fn init_windows_process(emu: &mut Emu) -> Result<(), MwemuError> {
    emu.init_win32(false, false)?;

    // Raw Windows code runs in a process with a main image in the PEB.
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
    Ok(())
}
