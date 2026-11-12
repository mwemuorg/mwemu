use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn SHParseDisplayName(emu: &mut emu::Emu) {
    let psz_name = emu.regs().rcx;
    let pbc = emu.regs().rdx;
    let ppidl = emu.regs().r8;
    let sfgao_in = emu.regs().r9;
    let psfgao_out = WinApiAbi::arg(emu, 4);
    let name = emu.maps.read_wide_string(psz_name);
    let hndl = helper::handler_create(&format!("pidl://{}", name));
    if ppidl != 0 {
        emu.maps.write_qword(ppidl, hndl);
    }
    if psfgao_out != 0 {
        emu.maps.write_dword(psfgao_out, sfgao_in as u32);
    }
    log_red!(emu, "shell32!SHParseDisplayName '{}' =0x{:x}", name, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
