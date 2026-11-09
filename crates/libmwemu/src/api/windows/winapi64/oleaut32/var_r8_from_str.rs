use crate::emu;

pub fn VarR8FromStr(emu: &mut emu::Emu) {
    let str_in = emu.regs().rcx;
    let _lcid = emu.regs().rdx as u32;
    let _flags = emu.regs().r8 as u32;
    let pdbl_out = emu.regs().r9;
    let s = if str_in != 0 {
        emu.maps.read_wide_string(str_in)
    } else {
        String::new()
    };
    let val = s.trim().parse::<f64>().unwrap_or(0.0);
    log_red!(
        emu,
        "oleaut32!VarR8FromStr '{}' => {} out:0x{:x}",
        s,
        val,
        pdbl_out
    );
    if pdbl_out != 0 {
        emu.maps.write_qword(pdbl_out, val.to_bits());
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
