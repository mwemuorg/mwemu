use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::user32::exports;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "AddClipboardFormatListener" => exports::AddClipboardFormatListener(emu),
        "ChangeClipboardChain" => exports::ChangeClipboardChain(emu),
        "CheckProcessForClipboardAccess" => exports::CheckProcessForClipboardAccess(emu),
        "CloseClipboard" => exports::CloseClipboard(emu),
        "CountClipboardFormats" => exports::CountClipboardFormats(emu),
        "EmptyClipboard" => exports::EmptyClipboard(emu),
        "EnumClipboardFormats" => exports::EnumClipboardFormats(emu),
        "GetClipboardAccessToken" => exports::GetClipboardAccessToken(emu),
        "GetClipboardData" => exports::GetClipboardData(emu),
        "GetClipboardFormatNameA" => exports::GetClipboardFormatNameA(emu),
        "GetClipboardFormatNameW" => exports::GetClipboardFormatNameW(emu),
        "GetClipboardMetadata" => exports::GetClipboardMetadata(emu),
        "GetClipboardOwner" => exports::GetClipboardOwner(emu),
        "GetClipboardSequenceNumber" => exports::GetClipboardSequenceNumber(emu),
        "GetClipboardViewer" => exports::GetClipboardViewer(emu),
        "GetOpenClipboardWindow" => exports::GetOpenClipboardWindow(emu),
        "GetPriorityClipboardFormat" => exports::GetPriorityClipboardFormat(emu),
        "IsClipboardFormatAvailable" => exports::IsClipboardFormatAvailable(emu),
        "OpenClipboard" => exports::OpenClipboard(emu),
        "RegisterClipboardFormatA" => exports::RegisterClipboardFormatA(emu),
        "RegisterClipboardFormatW" => exports::RegisterClipboardFormatW(emu),
        "RemoveClipboardFormatListener" => exports::RemoveClipboardFormatListener(emu),
        "SetClipboardData" => exports::SetClipboardData(emu),
        "SetClipboardViewer" => exports::SetClipboardViewer(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
