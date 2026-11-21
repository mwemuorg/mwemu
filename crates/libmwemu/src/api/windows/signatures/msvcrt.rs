use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "_lock" | "_unlock" => signature!(Void,1,1; Dword(lock_number)),
        "set_unexpected" => signature!(Generic, 1, 1; Pointer(handler)),
        "set_terminate" => signature!(Generic, 1, 1; Pointer(handler)),
        "_set_new_mode" => signature!(Generic, 1, 1; Dword(mode)),
        "_set_new_handler" => signature!(Generic, 1, 1; Pointer(handler)),
        "__setusermatherr" => signature!(Void, 1, 1; Pointer(handler)),
        "_set_se_translator" => signature!(Generic, 1, 1; Pointer(translator)),
        "_set_thread_local_invalid_parameter_handler" => {
            signature!(Generic, 1, 1; Pointer(handler))
        }
        "_set_invalid_parameter_handler" => signature!(Generic, 1, 1; Pointer(handler)),
        "_set_fmode" => signature!(Generic, 1, 1; Dword(mode)),
        "_set_error_mode" => signature!(Generic, 1, 1; Dword(mode)),
        "__set_app_type" => signature!(Void, 1, 1; Dword(app_type)),
        "_set_app_type" => signature!(Void, 1, 1; Dword(app_type)),
        "__CppXcptFilter" => {
            signature!(Generic, 2, 2; Dword(exception_code), Pointer(exception_pointers))
        }
        "_XcptFilter" => {
            signature!(Generic, 2, 2; Dword(exception_code), Pointer(exception_pointers))
        }
        "_except_handler4_common" => {
            signature!(Generic, 6, 6; Pointer(cookie), Pointer(check_cookie), Pointer(exception_record), Pointer(frame), Pointer(context), Pointer(dispatcher))
        }
        "_except_handler3" | "_except_handler4" | "__CxxFrameHandler" | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => {
            signature!(Generic, 4, 4; Pointer(exception_record), Pointer(frame), Pointer(context), Pointer(dispatcher))
        }
        "_CxxThrowException" => {
            signature!(Void, 2, 2; Pointer(exception_object), Pointer(throw_info))
        }
        "signal" => signature!(Generic, 2, 2; Dword(signal), Pointer(handler)),
        "sprintf" => signature!(Generic, 2, 2; Pointer(buffer), AnsiString(format)),
        "swprintf" => signature!(Generic, 2, 2; Pointer(buffer), WideString(format)),
        "snprintf" | "_snprintf" | "sprintf_s" | "_sprintf_p" => {
            signature!(Generic, 3, 3; Pointer(buffer), Pointer(capacity), AnsiString(format))
        }
        "_snprintf_s" => {
            signature!(Generic, 4, 4; Pointer(buffer), Pointer(capacity), Pointer(max_count), AnsiString(format))
        }
        "sscanf" | "sscanf_s" => signature!(Generic, 2, 2; AnsiString(input), AnsiString(format)),
        "_sscanf_s_l" => {
            signature!(Generic, 3, 3; AnsiString(input), AnsiString(format), Pointer(locale))
        }
        "vsprintf" => {
            signature!(Generic, 3, 3; Pointer(buffer), AnsiString(format), Pointer(arguments))
        }
        "vsnprintf" | "_vsnprintf" => {
            signature!(Generic, 4, 4; Pointer(buffer), Pointer(capacity), AnsiString(format), Pointer(arguments))
        }
        "_sprintf_l" => {
            signature!(Generic, 3, 3; Pointer(buffer), AnsiString(format), Pointer(locale))
        }
        "vwprintf" => signature!(Generic, 2, 2; WideString(format), Pointer(arguments)),
        "wprintf" => signature!(Generic, 1, 1; WideString(format)),
        "__stdio_common_vsprintf" | "__stdio_common_vsprintf_s" => {
            signature!(Generic, 6, 7; Qword(options), Pointer(buffer), Pointer(capacity), AnsiString(format), Pointer(locale), Pointer(arguments))
        }
        "__stdio_common_vsscanf" => {
            signature!(Generic, 6, 7; Qword(options), Pointer(input), Pointer(input_length), AnsiString(format), Pointer(locale), Pointer(arguments))
        }
        "setvbuf" => {
            signature!(Generic, 4, 4; Pointer(stream), Pointer(buffer), Dword(mode), Pointer(size))
        }
        "_setmode" => signature!(Generic, 2, 2; Dword(fd), Dword(mode)),
        "_localtime64" | "localtime" => signature!(Generic, 1, 1; Pointer(time)),
        "wcsftime" => {
            signature!(Generic, 4, 4; Pointer(buffer), Pointer(capacity), WideString(format), Pointer(time))
        }
        "_wsystem" => signature!(Generic, 1, 1; WideString(command)),
        _ => None,
    }
}
