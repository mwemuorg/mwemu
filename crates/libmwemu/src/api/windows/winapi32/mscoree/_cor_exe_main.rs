use super::parse_dotnet_metadata;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn _CorExeMain(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(
        emu,
        "mscoree!_CorExeMain  .NET CLR entry — parsing metadata"
    );
    parse_dotnet_metadata(emu);
    call.set_return(emu, 0);
    emu.stop();
}
