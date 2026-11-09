use super::parse_dotnet_metadata;
use crate::emu;

pub fn _CorExeMain(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "mscoree!_CorExeMain  .NET CLR entry — parsing metadata"
    );
    parse_dotnet_metadata(emu);
    emu.regs_mut().rax = 0;
    emu.stop();
}
