use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{heap, memory};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

const QUOTE: u16 = b'"' as u16;
const SLASH: u16 = b'\\' as u16;

fn blank(unit: &u16) -> bool {
    *unit == b' ' as u16 || *unit == b'\t' as u16
}

fn split(line: &[u16]) -> Vec<Vec<u16>> {
    let (first, tail) = if let Some(quoted) = line.strip_prefix(&[QUOTE]) {
        let end = quoted
            .iter()
            .position(|&unit| unit == QUOTE)
            .unwrap_or(quoted.len());
        (&quoted[..end], quoted.get(end + 1..).unwrap_or_default())
    } else {
        let end = line.iter().position(blank).unwrap_or(line.len());
        line.split_at(end)
    };
    let mut result = vec![first.to_vec()];
    let mut tail = tail.iter().copied().peekable();
    while tail.peek().is_some() {
        while tail.next_if(blank).is_some() {}
        if tail.peek().is_none() {
            break;
        }
        let mut word = Vec::new();
        let mut quoted = false;
        while let Some(&unit) = tail.peek() {
            if blank(&unit) && !quoted {
                break;
            }
            let mut slashes = 0;
            while tail.next_if_eq(&SLASH).is_some() {
                slashes += 1;
            }
            if tail.next_if_eq(&QUOTE).is_none() {
                word.extend(std::iter::repeat_n(SLASH, slashes));
                if let Some(ch) = tail.next_if(|ch| quoted || !blank(ch)) {
                    word.push(ch);
                }
                continue;
            }
            word.extend(std::iter::repeat_n(SLASH, slashes / 2));
            if slashes % 2 == 1 {
                word.push(QUOTE);
                continue;
            }
            let mut quotes = 1 + usize::from(quoted);
            while tail.next_if_eq(&QUOTE).is_some() {
                quotes += 1;
            }
            word.extend(std::iter::repeat_n(QUOTE, quotes / 3));
            quoted = quotes % 3 == 1;
        }
        result.push(word);
    }
    result
}

fn allocate(emu: &mut Emu, arguments: &[Vec<u16>]) -> Option<u64> {
    let width = memory::pointer_size(emu);
    let strings_at = width * (arguments.len() as u64 + 1);
    let bytes = arguments
        .iter()
        .map(|arg| (arg.len() as u64 + 1) * 2)
        .sum::<u64>();
    let array = heap::allocate(emu, 0, strings_at + bytes, 0)?;
    let mut text = array + strings_at;
    for (index, arg) in arguments.iter().enumerate() {
        let encoded: Vec<_> = arg
            .iter()
            .copied()
            .chain([0])
            .flat_map(u16::to_le_bytes)
            .collect();
        if !memory::write_pointer(emu, array + index as u64 * width, text)
            || !emu.maps.write_bytes(text, &encoded)
        {
            heap::heap_free(emu, 0, array);
            return None;
        }
        text += encoded.len() as u64;
    }
    memory::write_pointer(emu, array + arguments.len() as u64 * width, 0);
    Some(array)
}

pub fn CommandLineToArgvW(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [command, count] = api_try!(emu, call.args(emu));
    let result = (|| {
        if count == 0 || !emu.maps.validate_write_range(count, 4) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let line = Encoding::Wide.read(emu, command)?;
        let arguments = if line.is_empty() {
            let image = if emu.cfg.filename.is_empty() {
                &emu.cfg.exe_name
            } else {
                &emu.cfg.filename
            };
            vec![image.encode_utf16().collect()]
        } else {
            split(&line)
        };
        let argv = allocate(emu, &arguments).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        emu.maps.write_dword(count, arguments.len() as u32);
        Ok(argv)
    })();
    log_red!(
        emu,
        "shell32!CommandLineToArgvW command_line={command:#x} argument_count={count:#x} result={result:?}"
    );
    let argv = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, argv);
    call.cleanup_args(emu, 2);
}
