use crate::emu;
//use crate::serialization;
use crate::winapi::winapi64;

//use crate::windows::constants;
//use crate::winapi::helper;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PathCombineA" => PathCombineA(emu),
        "PathCombineW" => PathCombineW(emu),
        "CharLowerBuffW" => CharLowerBuffW(emu),
        "IsCharAlphaNumericA" => IsCharAlphaNumericA(emu),
        "GetTokenInformation" => GetTokenInformation(emu),
        "GetFileVersionInfoSizeA" => GetFileVersionInfoSizeA(emu),
        "GetFileVersionInfoA" => GetFileVersionInfoA(emu),
        "VerQueryValueA" => VerQueryValueA(emu),
        "_initterm_e" => _initterm_e(emu),
        "_initterm" => _initterm(emu),
        "exit" => exit(emu),
        "_exit" => _exit(emu),
        "atexit" => atexit(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "LocalAlloc" => LocalAlloc(emu),

        _ => {
            // kernelbase is bridge to kernel32
            winapi64::kernel32::gateway(addr, emu);
        }
    }

    String::new()
}

pub fn PathCombineA(emu: &mut emu::Emu) {
    let dst: u64 = emu.regs().rcx;
    let dir = emu.regs().rdx;
    let file = emu.regs().r8;

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = emu.maps.read_string(dir);
    }
    if file > 0 {
        path2 = emu.maps.read_string(file);
    }

    log_red!(
        emu,
        "kernelbase!PathCombineA path1: {} path2: {}",
        path1,
        path2
    );

    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps.write_string(dst, &format!("{}\\{}", path1, path2));
    }

    emu.regs_mut().rax = dst;
}

pub fn PathCombineW(emu: &mut emu::Emu) {
    let dst: u64 = emu.regs().rcx;
    let dir = emu.regs().rdx;
    let file = emu.regs().r8;

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = emu.maps.read_wide_string(dir);
    }
    if file > 0 {
        path2 = emu.maps.read_wide_string(file);
    }

    log_red!(
        emu,
        "kernelbase!PathCombineW path1: {} path2: {}",
        path1,
        path2
    );

    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps
            .write_wide_string(dst, &format!("{}\\{}", path1, path2));
    }

    emu.regs_mut().rax = dst;
}

pub fn IsCharAlphaNumericA(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8 as char;

    log_red!(emu, "kernelbase!IsCharAlphaNumericA char: {}", c);

    emu.regs_mut().rax = if c.is_ascii_alphanumeric() { 1 } else { 0 };
}

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rdx;
    let token_information_class = emu.regs().rcx;
    let token_information = emu.regs().r8;
    let token_information_length = emu.regs().r9;
    let return_length = emu.maps.read_qword(emu.regs().rsp + 0x20);

    log_red!(
        emu,
        "kernelbase!GetTokenInformation token_information_class: 0x{:x}",
        token_information_class
    );

    emu.regs_mut().rax = 1;
}

/*
DWORD GetFileVersionInfoSizeA(
  [in]            LPCSTR  lptstrFilename,
  [out, optional] LPDWORD lpdwHandle
);
*/
fn GetFileVersionInfoSizeA(emu: &mut emu::Emu) {
    let lptstr_filename = emu.regs().rcx;
    let lpdw_handle = emu.regs().rdx as usize;

    let filename = if lptstr_filename > 0 {
        emu.maps.read_string(lptstr_filename)
    } else {
        "unknown".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!GetFileVersionInfoSizeA filename: {} lpdw_handle: 0x{:x}",
        emu.pos,
        filename,
        lpdw_handle
    );

    if filename == "comctl32.dll" {
        let dll_path = format!("{}/comctl32.dll", emu.cfg.maps_folder);
        let metadata = std::fs::metadata(dll_path).unwrap();
        let file_size = metadata.len() as u64;
        emu.regs_mut().rax = file_size;
    } else {
        panic!("TODO: {}", filename);
    }
}

/*
BOOL GetFileVersionInfoA(
  [in]  LPCSTR lptstrFilename,
        DWORD  dwHandle,
  [in]  DWORD  dwLen,
  [out] LPVOID lpData
);
*/
fn GetFileVersionInfoA(emu: &mut emu::Emu) {
    let lptstr_filename = emu.regs().rcx;
    let dw_handle = emu.regs().rdx as usize;
    let dw_len = emu.regs().r8 as usize;
    let lp_data = emu.regs().r9 as usize;

    let filename = if lptstr_filename > 0 {
        emu.maps.read_string(lptstr_filename)
    } else {
        "unknown".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!GetFileVersionInfoA filename: {} dw_handle: 0x{:x} dw_len: 0x{:x} lp_data: 0x{:x}",
        emu.pos,
        filename,
        dw_handle,
        dw_len,
        lp_data
    );

    if filename == "comctl32.dll" {
        use crate::windows::structures::{VS_FIXEDFILEINFO, VS_VERSIONINFO};

        let mut version_info = VS_VERSIONINFO::new();

        // Set comctl32.dll specific values based on the actual file
        version_info.value = VS_FIXEDFILEINFO {
            dw_signature: 0xFEEF04BD,
            dw_struc_version: 0x00010000,
            dw_file_version_ms: 0x0006000A,    // 6.10
            dw_file_version_ls: 0x585D11BD,    // 22621.4541
            dw_product_version_ms: 0x000A0000, // 10.0
            dw_product_version_ls: 0x585D11BD, // 22621.4541
            dw_file_flags_mask: 0x0000003F,
            dw_file_flags: 0x00000000,
            dw_file_os: 0x00040004,   // VOS_NT_WINDOWS32
            dw_file_type: 0x00000002, // VFT_DLL
            dw_file_subtype: 0x00000000,
            dw_file_date_ms: 0x00000000,
            dw_file_date_ls: 0x00000000,
        };

        version_info.write(emu, lp_data as u64);

        emu.regs_mut().rax = 1; // Success
    } else {
        panic!("TODO: {}", filename);
    }
}

/*
BOOL VerQueryValueA(
  [in]  LPCVOID pBlock,
  [in]  LPCSTR  lpSubBlock,
  [out] LPVOID  *lplpBuffer,
  [out] PUINT   puLen
);
*/
fn VerQueryValueA(emu: &mut emu::Emu) {
    let p_block = emu.regs().rcx as usize;
    let lp_sub_block = emu.regs().rdx;
    let lplp_buffer = emu.regs().r8 as usize;
    let pu_len = emu.regs().r9 as usize;

    let sub_block = if lp_sub_block > 0 {
        emu.maps.read_string(lp_sub_block)
    } else {
        "\\".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!VerQueryValueA p_block: 0x{:x} lp_sub_block: {} lplp_buffer: 0x{:x} pu_len: 0x{:x}",
        emu.pos,
        p_block,
        sub_block,
        lplp_buffer,
        pu_len
    );

    if sub_block == "\\" {
        // Root query returns pointer to VS_FIXEDFILEINFO
        // The VS_FIXEDFILEINFO starts at offset 0x28 in the version block
        let fixed_info_ptr = (p_block + 0x28) as u64;
        emu.maps.write_qword(lplp_buffer as u64, fixed_info_ptr);
        emu.maps.write_dword(pu_len as u64, 52); // Size of VS_FIXEDFILEINFO
        emu.regs_mut().rax = 1;
    } else if sub_block.starts_with("\\StringFileInfo\\") {
        // String queries - allocate and return string data
        let string_data = match sub_block.as_str() {
            "\\StringFileInfo\\040904B0\\CompanyName" => "Microsoft Corporation\0",
            "\\StringFileInfo\\040904B0\\FileDescription" => "User Experience Controls Library\0",
            "\\StringFileInfo\\040904B0\\FileVersion" => "6.10 (WinBuild.160101.0800)\0",
            "\\StringFileInfo\\040904B0\\InternalName" => "comctl32\0",
            "\\StringFileInfo\\040904B0\\LegalCopyright" => {
                "© Microsoft Corporation. All rights reserved.\0"
            }
            "\\StringFileInfo\\040904B0\\OriginalFilename" => "comctl32.DLL\0",
            "\\StringFileInfo\\040904B0\\ProductName" => "Microsoft® Windows® Operating System\0",
            "\\StringFileInfo\\040904B0\\ProductVersion" => "10.0.22621.4541\0",
            _ => "\0",
        };

        let string_addr = emu
            .maps
            .alloc(string_data.len() as u64)
            .expect("out of memory");
        emu.maps.write_string(string_addr, string_data);
        emu.maps.write_qword(lplp_buffer as u64, string_addr);
        emu.maps
            .write_dword(pu_len as u64, string_data.len() as u32);
        emu.regs_mut().rax = 1;
    } else if sub_block == "\\VarFileInfo\\Translation" {
        // Translation array
        let trans_addr = emu.maps.alloc(4).expect("out of memory");
        emu.maps.write_dword(trans_addr, 0x04B00409); // Language and codepage
        emu.maps.write_qword(lplp_buffer as u64, trans_addr);
        emu.maps.write_dword(pu_len as u64, 4);
        emu.regs_mut().rax = 1;
    } else {
        log::trace!("VerQueryValueA: Unknown sub_block: {}", sub_block);
        emu.regs_mut().rax = 0; // Failure
    }
}

fn _initterm_e(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!_initterm_e");
    emu.regs_mut().rax = 0;
}

fn _initterm(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!_initterm");
    emu.regs_mut().rax = 0;
}

fn exit(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!exit");
    panic!("exit called");
}

fn _exit(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!_exit");
    panic!("_exit called");
}

fn atexit(emu: &mut emu::Emu) {
    let fptr = emu.regs().rcx;
    log_red!(emu, "kernelbase!atexit fptr: 0x{:x}", fptr);
    emu.regs_mut().rax = 0;
}

/*
DWORD CharLowerBuffW(
  [in, out] LPWSTR lpsz,
  [in]      DWORD  cchLength
);
*/
pub fn CharLowerBuffW(emu: &mut emu::Emu) {
    let lpsz = emu.regs().rcx; // Buffer pointer (LPWSTR)
    let cch_length = emu.regs().rdx; // Length in characters (DWORD)

    log_red!(
        emu,
        "kernelbase!CharLowerBuffW lpsz: 0x{:x} cchLength: {}",
        lpsz,
        cch_length
    );

    if lpsz == 0 || cch_length == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let mut processed_count = 0;

    // Process each character in the buffer
    for i in 0..cch_length {
        let char_addr = lpsz + (i * 2); // Each wide character is 2 bytes

        if let Some(wide_char) = emu.maps.read_word(char_addr) {
            // Convert UTF-16 code unit to char for processing
            if let Some(unicode_char) = char::from_u32(wide_char as u32) {
                // Convert to lowercase
                let lowercase_char = unicode_char.to_lowercase().next().unwrap_or(unicode_char);

                // Convert back to UTF-16 code unit
                let mut utf16_buf = [0u16; 2];
                let utf16_encoded = lowercase_char.encode_utf16(&mut utf16_buf);

                // Write the lowercase character back (take first code unit for BMP characters)
                let lowercase_code_unit = utf16_encoded[0];
                emu.maps.write_word(char_addr, lowercase_code_unit);

                processed_count += 1;
            } else {
                // Invalid Unicode, but still count as processed
                processed_count += 1;
            }
        } else {
            // Couldn't read memory, break early
            break;
        }
    }

    log_red!(
        emu,
        "CharLowerBuffW processed {} characters",
        processed_count
    );

    // Return the number of characters processed
    emu.regs_mut().rax = processed_count;
}

fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let ptr1 = emu.regs().rcx;

    log::trace!(
        "{}** {} kernelbase!SetUnhandledExceptionFilter 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr1,
        emu.colors.nc
    );

    emu.set_uef(ptr1);
    emu.regs_mut().rax = 0;
}

fn LocalAlloc(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let size = emu.regs().rdx;

    let addr = emu.maps.alloc(size).unwrap_or_default();

    log_red!(emu, "kernelbase!LocalAlloc {} =0x{:x}", size, addr);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = addr;
}
