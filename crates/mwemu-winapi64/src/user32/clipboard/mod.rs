use crate::emu::Emu;
use crate::winapi::winapi64::user32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AddClipboardFormatListener" => user32::AddClipboardFormatListener(emu),
        "ChangeClipboardChain" => user32::ChangeClipboardChain(emu),
        "CheckProcessForClipboardAccess" => user32::CheckProcessForClipboardAccess(emu),
        "CloseClipboard" => user32::CloseClipboard(emu),
        "CountClipboardFormats" => user32::CountClipboardFormats(emu),
        "EmptyClipboard" => user32::EmptyClipboard(emu),
        "EnumClipboardFormats" => user32::EnumClipboardFormats(emu),
        "GetClipboardAccessToken" => user32::GetClipboardAccessToken(emu),
        "GetClipboardData" => user32::GetClipboardData(emu),
        "GetClipboardFormatNameA" => user32::GetClipboardFormatNameA(emu),
        "GetClipboardFormatNameW" => user32::GetClipboardFormatNameW(emu),
        "GetClipboardMetadata" => user32::GetClipboardMetadata(emu),
        "GetClipboardOwner" => user32::GetClipboardOwner(emu),
        "GetClipboardSequenceNumber" => user32::GetClipboardSequenceNumber(emu),
        "GetClipboardViewer" => user32::GetClipboardViewer(emu),
        "GetOpenClipboardWindow" => user32::GetOpenClipboardWindow(emu),
        "GetPriorityClipboardFormat" => user32::GetPriorityClipboardFormat(emu),
        "GetUpdatedClipboardFormats" => user32::GetUpdatedClipboardFormats(emu),
        "IsClipboardFormatAvailable" => user32::IsClipboardFormatAvailable(emu),
        "OpenClipboard" => user32::OpenClipboard(emu),
        "RegisterClipboardFormatA" => user32::RegisterClipboardFormatA(emu),
        "RegisterClipboardFormatW" => user32::RegisterClipboardFormatW(emu),
        "RemoveClipboardFormatListener" => user32::RemoveClipboardFormatListener(emu),
        "SetClipboardData" => user32::SetClipboardData(emu),
        "SetClipboardViewer" => user32::SetClipboardViewer(emu),
        _ => return false,
    }
    true
}
