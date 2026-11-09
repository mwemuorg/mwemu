use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod win_http_close_handle;
use win_http_close_handle::WinHttpCloseHandle;
mod win_http_connect;
use win_http_connect::WinHttpConnect;
mod win_http_open;
use win_http_open::WinHttpOpen;
mod win_http_open_request;
use win_http_open_request::WinHttpOpenRequest;
mod win_http_read_data;
use win_http_read_data::WinHttpReadData;
mod win_http_receive_response;
use win_http_receive_response::WinHttpReceiveResponse;
mod win_http_send_request;
use win_http_send_request::WinHttpSendRequest;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "WinHttpOpen" => WinHttpOpen(emu),
        "WinHttpConnect" => WinHttpConnect(emu),
        "WinHttpOpenRequest" => WinHttpOpenRequest(emu),
        "WinHttpSendRequest" => WinHttpSendRequest(emu),
        "WinHttpReceiveResponse" => WinHttpReceiveResponse(emu),
        "WinHttpReadData" => WinHttpReadData(emu),
        "WinHttpCloseHandle" => WinHttpCloseHandle(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}
