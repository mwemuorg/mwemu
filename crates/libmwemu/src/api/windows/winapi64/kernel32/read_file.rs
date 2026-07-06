use crate::emu;
use crate::emu::object_handle::file_handle::INVALID_HANDLE_VALUE;

pub fn ReadFile(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx as usize; // Handle to the file
    let lp_buffer = emu.regs().rdx; // Buffer to receive dat
    let n_number_of_bytes_to_read = emu.regs().r8 as u32; // Number of bytes to read
    let lp_number_of_bytes_read = emu.regs().r9; // Pointer to store bytes read
    let lp_overlapped = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!ReadFile cannot read the overlapped");

    log_red!(
        emu,
        "** {} kernel32!ReadFile hFile: 0x{:x} lpBuffer: 0x{:x} nNumberOfBytesToRead: {} lpNumberOfBytesRead: 0x{:x} lpOverlapped: 0x{:x}",
        emu.pos,
        h_file,
        lp_buffer,
        n_number_of_bytes_to_read,
        lp_number_of_bytes_read,
        lp_overlapped
    );

    // Check if handle is valid (not INVALID_HANDLE_VALUE)
    if h_file == INVALID_HANDLE_VALUE {
        log_red!(
            emu,
            "** {} kernel32!ReadFile ERROR: Invalid handle (INVALID_HANDLE_VALUE)",
            emu.pos
        );
        emu.last_error = 6; // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0; // FALSE
        return;
    }

    // Get the file handle from the handle management system
    let file_handle_ref = match emu.handle_management.get_mut_file_handle(h_file as u32) {
        Some(fh) => fh,
        None => {
            log_red!(
                emu,
                "** {} kernel32!ReadFile ERROR: Handle 0x{:x} not found in handle table",
                emu.pos,
                h_file
            );
            emu.last_error = 6; // ERROR_INVALID_HANDLE
            emu.regs_mut().rax = 0; // FALSE
            return;
        }
    };

    // Check if the handle is valid and not a directory
    if !file_handle_ref.is_valid() {
        log_red!(
            emu,
            "** {} kernel32!ReadFile ERROR: Handle 0x{:x} is invalid",
            emu.pos,
            h_file
        );
        emu.last_error = 6; // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0; // FALSE
        return;
    }

    if file_handle_ref.is_dir() {
        log_red!(
            emu,
            "** {} kernel32!ReadFile ERROR: Cannot read from directory handle",
            emu.pos
        );
        emu.last_error = 87; // ERROR_INVALID_PARAMETER
        emu.regs_mut().rax = 0; // FALSE
        return;
    }

    // Check if buffer pointer is valid
    if lp_buffer == 0 && n_number_of_bytes_to_read > 0 {
        log_red!(
            emu,
            "** {} kernel32!ReadFile ERROR: Invalid buffer pointer",
            emu.pos
        );
        emu.last_error = 87; // ERROR_INVALID_PARAMETER
        emu.regs_mut().rax = 0; // FALSE
        return;
    }

    // Prepare buffer for reading
    let mut buffer = vec![0u8; n_number_of_bytes_to_read as usize];

    // Read from the file handle
    let bytes_read_result = file_handle_ref.read(&mut buffer);

    let bytes_read = match bytes_read_result {
        Ok(bytes) => {
            log_red!(
                emu,
                "** {} kernel32!ReadFile SUCCESS: Read {} bytes",
                emu.pos,
                bytes
            );
            // Write the data back to the emulator's memory
            if bytes > 0
                && !emu.maps.write_bytes(lp_buffer, &buffer) {
                    log_red!(
                        emu,
                        "** {} kernel32!ReadFile ERROR: Failed to write data to buffer at 0x{:x}",
                        emu.pos,
                        lp_buffer
                    );
                    emu.last_error = 14; // ERROR_OUTOFMEMORY
                    emu.regs_mut().rax = 0; // FALSE
                    return;
                }
            bytes
        }
        Err(e) => {
            let error_code = match e.kind() {
                std::io::ErrorKind::NotFound => 2,         // ERROR_FILE_NOT_FOUND
                std::io::ErrorKind::PermissionDenied => 5, // ERROR_ACCESS_DENIED
                std::io::ErrorKind::InvalidInput => 87,    // ERROR_INVALID_PARAMETER
                std::io::ErrorKind::UnexpectedEof => 0,    // EOF - not an error
                _ => 1,                                    // ERROR_INVALID_FUNCTION
            };

            log_red!(
                emu,
                "** {} kernel32!ReadFile ERROR: Read failed - {}",
                emu.pos,
                e
            );
            emu.last_error = error_code;

            // If it's EOF, we still return success but with 0 bytes read
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                0
            } else {
                emu.regs_mut().rax = 0; // FALSE
                return;
            }
        }
    };

    // Write the number of bytes read to the output parameter
    if lp_number_of_bytes_read != 0
        && !emu
            .maps
            .write_dword(lp_number_of_bytes_read, bytes_read as u32)
        {
            log_red!(
                emu,
                "** {} kernel32!ReadFile ERROR: Failed to write bytes read count to 0x{:x}",
                emu.pos,
                lp_number_of_bytes_read
            );
            emu.last_error = 14; // ERROR_OUTOFMEMORY
            emu.regs_mut().rax = 0; // FALSE
            return;
        }

    // Success!
    emu.last_error = 0; // NO_ERROR
    emu.regs_mut().rax = 1; // TRUE
}
