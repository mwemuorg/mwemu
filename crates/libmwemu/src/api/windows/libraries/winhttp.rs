use crate::api::dispatch_api;
use crate::emu::Emu;
pub use crate::winapi::libraries::http::{
    WinHttpCloseHandle, WinHttpConnect, WinHttpOpen, WinHttpOpenRequest, WinHttpReadData,
    WinHttpReceiveResponse, WinHttpSendRequest,
};

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "WinHttpOpen" => WinHttpOpen(emu),
        "WinHttpConnect" => WinHttpConnect(emu),
        "WinHttpOpenRequest" => WinHttpOpenRequest(emu),
        "WinHttpSendRequest" => WinHttpSendRequest(emu),
        "WinHttpReceiveResponse" => WinHttpReceiveResponse(emu),
        "WinHttpReadData" => WinHttpReadData(emu),
        "WinHttpCloseHandle" => WinHttpCloseHandle(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
