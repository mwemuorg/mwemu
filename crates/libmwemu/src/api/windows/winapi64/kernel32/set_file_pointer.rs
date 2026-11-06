use std::io::SeekFrom;

use crate::emu;
use crate::windows::constants;

const FILE_BEGIN: u64 = 0;
const FILE_CURRENT: u64 = 1;
const FILE_END: u64 = 2;
const INVALID_SET_FILE_POINTER: u64 = 0xffff_ffff;

pub fn SetFilePointer(emu: &mut emu::Emu) {
    let hfile = emu.regs().rcx;
    let dist_low = emu.regs().rdx as u32;
    let dist_high_ptr = emu.regs().r8;
    let method = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!SetFilePointer hFile:0x{:x} dist:{} high_ptr:0x{:x} method:{}",
        hfile,
        dist_low as i32,
        dist_high_ptr,
        method
    );

    let distance = if dist_high_ptr != 0 {
        let high = emu.maps.read_dword(dist_high_ptr).unwrap_or(0) as u64;
        ((high << 32) | dist_low as u64) as i64
    } else {
        dist_low as i32 as i64
    };

    match seek(emu, hfile, distance, method) {
        Ok(pos) => {
            if dist_high_ptr != 0 {
                emu.maps.write_dword(dist_high_ptr, (pos >> 32) as u32);
            }
            emu.regs_mut().rax = pos & 0xffff_ffff;
        }
        Err(error) => {
            emu.set_last_error(error);
            emu.regs_mut().rax = INVALID_SET_FILE_POINTER;
        }
    }
}

pub fn SetFilePointerEx(emu: &mut emu::Emu) {
    let hfile = emu.regs().rcx;
    let distance = emu.regs().rdx as i64;
    let new_pos_ptr = emu.regs().r8;
    let method = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!SetFilePointerEx h:0x{:x} dist:{} method:{}",
        hfile,
        distance,
        method
    );

    match seek(emu, hfile, distance, method) {
        Ok(pos) => {
            if new_pos_ptr != 0 {
                emu.maps.write_qword(new_pos_ptr, pos);
            }
            emu.regs_mut().rax = 1;
        }
        Err(error) => {
            emu.set_last_error(error);
            emu.regs_mut().rax = 0;
        }
    }
}

fn seek(emu: &mut emu::Emu, hfile: u64, distance: i64, method: u64) -> Result<u64, u64> {
    let target = match method {
        FILE_BEGIN if distance < 0 => return Err(constants::ERROR_NEGATIVE_SEEK),
        FILE_BEGIN => SeekFrom::Start(distance as u64),
        FILE_CURRENT => SeekFrom::Current(distance),
        FILE_END => SeekFrom::End(distance),
        _ => return Err(constants::ERROR_INVALID_PARAMETER),
    };
    let file = emu
        .handle_management
        .get_mut_file_handle(hfile as u32)
        .ok_or(constants::ERROR_INVALID_HANDLE)?;
    file.seek(target)
        .map_err(|_| constants::ERROR_NEGATIVE_SEEK)
}
