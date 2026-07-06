use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

pub fn execute(emu: &mut Emu, _ins: &Instruction) -> bool {
    // MSR <sysreg>, Xt — write Xt to system register
    // For now, ignore writes
    let _ = emu;
    log::trace!("MSR: ignoring write to system register");
    true
}
