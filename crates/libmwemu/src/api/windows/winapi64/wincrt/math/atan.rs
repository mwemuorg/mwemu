use crate::emu;
use crate::winapi::helper;

pub fn atan(emu: &mut emu::Emu, api: &str) {
    helper::advance_tick(emu, 1);
    let a = f64::from_bits(emu.regs().xmm0 as u64);
    let result = match api {
        "atan" => a.atan(),
        "cos" => a.cos(),
        "sin" => a.sin(),
        "tan" => a.tan(),
        "exp" => a.exp(),
        "log" => a.ln(),
        "log10" => a.log10(),
        "sqrt" => a.sqrt(),
        "ceil" => a.ceil(),
        "floor" => a.floor(),
        "fabs" => a.abs(),
        _ => {
            let b = f64::from_bits(emu.regs().xmm1 as u64);
            match api {
                "atan2" => a.atan2(b),
                "pow" => a.powf(b),
                "fmod" => a % b,
                _ => 0.0,
            }
        }
    };
    log_red!(emu, "wincrt!{} ({}) = {}", api, a, result);
    emu.regs_mut().xmm0 = result.to_bits() as u128;
    emu.regs_mut().rax = result.to_bits();
}
