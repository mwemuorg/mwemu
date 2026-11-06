use crate::{emu, windows::constants};

use super::module_path;

/*
DWORD GetModuleFileNameA(
  [in, optional] HMODULE hModule,
  [out]          LPSTR   lpFilename,
  [in]           DWORD   nSize
);
*/
pub fn GetModuleFileNameA(emu: &mut emu::Emu) {
    let module_handle = emu.regs().rcx;
    let lp_filename = emu.regs().rdx;
    let n_size = emu.regs().r8 as u32 as usize;

    let path = module_path(emu, module_handle);
    log_red!(
        emu,
        "kernel32!GetModuleFileNameA hModule: 0x{:x} nSize: {} = {:?}",
        module_handle,
        n_size,
        path
    );

    let Some(path) = path else {
        emu.set_last_error(constants::ERROR_MOD_NOT_FOUND);
        emu.regs_mut().rax = 0;
        return;
    };
    if n_size == 0 || lp_filename == 0 {
        emu.set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = 0;
        return;
    }
    if path.len() < n_size {
        emu.maps.write_string(lp_filename, &path);
        emu.regs_mut().rax = path.len() as u64;
    } else {
        // truncated to nSize - 1 characters plus the terminator
        emu.maps.write_string(lp_filename, &path[..n_size - 1]);
        emu.set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = n_size as u64;
    }
}
