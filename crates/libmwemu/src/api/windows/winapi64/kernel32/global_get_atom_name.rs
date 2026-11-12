use crate::emu;
use crate::winapi::common::strings;

pub fn GlobalGetAtomNameA(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameA", false);
}

fn global_get_atom_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let atom = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let name = format!("Atom#{:04X}", atom);
    log_red!(emu, "kernel32!{api} atom:0x{:x} => '{}'", atom, name);
    if buf != 0 && buf_size > 0 {
        strings::write_string(emu, buf, &name, wide);
        emu.regs_mut().rax = name.len() as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}

pub fn GlobalGetAtomNameW(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameW", true);
}
