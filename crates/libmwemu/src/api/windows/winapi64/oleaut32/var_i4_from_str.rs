use crate::emu;

pub fn VarI4FromStr(emu: &mut emu::Emu) {
    let str_in = emu.regs().rcx;
    let _lcid = emu.regs().rdx as u32;
    let _flags = emu.regs().r8 as u32;
    let pl_out = emu.regs().r9;
    let s = if str_in != 0 {
        emu.maps.read_wide_string(str_in)
    } else {
        String::new()
    };
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(
        emu,
        "oleaut32!VarI4FromStr '{}' => {} out:0x{:x}",
        s,
        val,
        pl_out
    );
    if pl_out != 0 {
        emu.maps.write_dword(pl_out, val as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
