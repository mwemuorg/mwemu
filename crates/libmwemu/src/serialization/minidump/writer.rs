use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::{LittleEndian, WriteBytesExt};
use minidump::format as md;

use super::context::{ThreadContextInput, build_thread_context};
use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::threading::context::ArchThreadState;

const MINIDUMP_HEADER_SIZE: u32 = 32;
const MINIDUMP_DIRECTORY_SIZE: u32 = 12;
const MINIDUMP_THREAD_SIZE: u32 = 48;
const MINIDUMP_MODULE_SIZE: u32 = 108;
const MINIDUMP_MEMORY_DESCRIPTOR_SIZE: u32 = 16;
const MINIDUMP_MEMORY_INFO_SIZE: u32 = 48;
const MINIDUMP_SYSTEM_INFO_SIZE: u32 = 56;
const MINIDUMP_FLAGS_FULL_MEMORY: u64 = 0x802;

#[derive(Clone, Copy)]
struct MemoryRegion<'a> {
    name: &'a str,
    base: u64,
    bytes: &'a [u8],
    permission: Permission,
}

#[derive(Clone, Copy)]
struct MemoryLocation {
    data_size: u32,
    rva: u32,
}

struct ModuleInfo {
    prefix: String,
    display_name: String,
    base: u64,
    size_of_image: u32,
}

struct StreamLayout {
    stream_type: u32,
    rva: u32,
    data: Vec<u8>,
    trailing_data: Vec<u8>,
}

pub struct MinidumpWriter;

impl MinidumpWriter {
    pub fn write_to_file(emu: &Emu, filename: &str) -> io::Result<()> {
        let regions = collect_memory_regions(emu);
        let modules = collect_modules(emu, &regions);
        let stream_count = 5u32;
        let payload_start = align4(MINIDUMP_HEADER_SIZE + (stream_count * MINIDUMP_DIRECTORY_SIZE));
        let memory_rva = payload_start;
        let (memory_stream, memory_trailing_data, memory_locations) =
            build_memory_list_stream(&regions, memory_rva)?;

        let memory_info_rva =
            align4(memory_rva + memory_stream.len() as u32 + memory_trailing_data.len() as u32);
        let memory_info_stream =
            build_memory_info_list_stream(&regions, &modules, memory_info_rva)?;

        let system_info_rva = align4(memory_info_rva + memory_info_stream.len() as u32);
        let system_info_stream = build_system_info_stream(emu, system_info_rva)?;

        let module_rva = align4(system_info_rva + system_info_stream.len() as u32);
        let (module_stream, module_trailing_data) = build_module_list_stream(&modules, module_rva)?;

        let thread_rva =
            align4(module_rva + module_stream.len() as u32 + module_trailing_data.len() as u32);
        let (thread_stream, thread_trailing_data) =
            build_thread_list_stream(emu, &regions, &memory_locations, thread_rva)?;

        let by_position = vec![
            StreamLayout {
                stream_type: md::MINIDUMP_STREAM_TYPE::MemoryListStream as u32,
                rva: memory_rva,
                data: memory_stream,
                trailing_data: memory_trailing_data,
            },
            StreamLayout {
                stream_type: md::MINIDUMP_STREAM_TYPE::MemoryInfoListStream as u32,
                rva: memory_info_rva,
                data: memory_info_stream,
                trailing_data: Vec::new(),
            },
            StreamLayout {
                stream_type: md::MINIDUMP_STREAM_TYPE::SystemInfoStream as u32,
                rva: system_info_rva,
                data: system_info_stream,
                trailing_data: Vec::new(),
            },
            StreamLayout {
                stream_type: md::MINIDUMP_STREAM_TYPE::ModuleListStream as u32,
                rva: module_rva,
                data: module_stream,
                trailing_data: module_trailing_data,
            },
            StreamLayout {
                stream_type: md::MINIDUMP_STREAM_TYPE::ThreadListStream as u32,
                rva: thread_rva,
                data: thread_stream,
                trailing_data: thread_trailing_data,
            },
        ];

        let directories = vec![
            directory_for_stream(&by_position[4]),
            directory_for_stream(&by_position[3]),
            directory_for_stream(&by_position[0]),
            directory_for_stream(&by_position[2]),
            directory_for_stream(&by_position[1]),
        ];

        let mut output = Vec::new();
        write_header(&mut output, stream_count)?;
        for directory in directories {
            write_directory(&mut output, &directory)?;
        }

        for stream in &by_position {
            pad_to_rva(&mut output, stream.rva)?;
            output.extend_from_slice(&stream.data);
            output.extend_from_slice(&stream.trailing_data);
        }

        let mut file = File::create(filename)?;
        file.write_all(&output)?;
        file.flush()?;
        Ok(())
    }
}

fn align4(value: u32) -> u32 {
    (value + 3) & !3
}

fn collect_memory_regions(emu: &Emu) -> Vec<MemoryRegion<'_>> {
    emu.maps
        .maps
        .iter()
        .filter_map(|(base, slab_key)| {
            let mem = emu.maps.mem_slab.get(*slab_key)?;
            if mem.size() == 0 {
                return None;
            }

            Some(MemoryRegion {
                name: mem.get_name(),
                base: *base,
                bytes: mem.get_bytes(),
                permission: mem.permission(),
            })
        })
        .collect()
}

fn collect_modules(emu: &Emu, regions: &[MemoryRegion<'_>]) -> Vec<ModuleInfo> {
    let mut modules = Vec::new();

    for region in regions {
        let Some(prefix) = region.name.strip_suffix(".pe") else {
            continue;
        };

        let size_end = regions
            .iter()
            .filter(|candidate| candidate.base >= region.base)
            .filter(|candidate| region_belongs_to_module(candidate.name, prefix))
            .map(|candidate| candidate.base + candidate.bytes.len() as u64)
            .max()
            .unwrap_or(region.base + region.bytes.len() as u64);

        modules.push(ModuleInfo {
            prefix: prefix.to_string(),
            display_name: display_name_for_module(emu, prefix),
            base: region.base,
            size_of_image: size_end.saturating_sub(region.base).min(u32::MAX as u64) as u32,
        });
    }

    modules
}

fn display_name_for_module(emu: &Emu, prefix: &str) -> String {
    let prefix_lc = prefix.to_ascii_lowercase();
    let main_stem = Path::new(&emu.filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_ascii_lowercase());
    let exe_name_stem = Path::new(&emu.cfg.exe_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_ascii_lowercase());

    if main_stem.as_deref() == Some(prefix_lc.as_str())
        && !emu.filename.is_empty() {
            return emu.filename.clone();
        }

    if exe_name_stem.as_deref() == Some(prefix_lc.as_str())
        && !emu.cfg.exe_name.is_empty() {
            return emu.cfg.exe_name.clone();
        }

    if !emu.cfg.maps_folder.is_empty() {
        let dll_path = Path::new(&emu.cfg.maps_folder).join(format!("{prefix}.dll"));
        if dll_path.exists() {
            return dll_path.to_string_lossy().into_owned();
        }

        let exe_path = Path::new(&emu.cfg.maps_folder).join(format!("{prefix}.exe"));
        if exe_path.exists() {
            return exe_path.to_string_lossy().into_owned();
        }
    }

    if main_stem.as_deref() == Some(prefix_lc.as_str())
        || exe_name_stem.as_deref() == Some(prefix_lc.as_str())
    {
        format!("{prefix}.exe")
    } else {
        format!("{prefix}.dll")
    }
}

fn region_belongs_to_module(name: &str, prefix: &str) -> bool {
    if name == format!("{prefix}.pe") {
        return true;
    }

    let Some(rest) = name.strip_prefix(prefix) else {
        return false;
    };

    rest.starts_with('.')
        || rest
            .chars()
            .next()
            .map(|c| c.is_ascii_hexdigit())
            .unwrap_or(false)
}

fn build_memory_list_stream(
    regions: &[MemoryRegion<'_>],
    base_rva: u32,
) -> io::Result<(Vec<u8>, Vec<u8>, BTreeMap<u64, MemoryLocation>)> {
    let header_size = 4 + ((regions.len() as u32) * MINIDUMP_MEMORY_DESCRIPTOR_SIZE);
    let mut next_data_rva = base_rva + header_size;
    let mut output = Vec::new();
    let mut trailing_data = Vec::new();
    let mut locations = BTreeMap::new();

    output.write_u32::<LittleEndian>(regions.len() as u32)?;

    for region in regions {
        let data_size = region.bytes.len().min(u32::MAX as usize) as u32;
        locations.insert(
            region.base,
            MemoryLocation {
                data_size,
                rva: next_data_rva,
            },
        );

        output.write_u64::<LittleEndian>(region.base)?;
        output.write_u32::<LittleEndian>(data_size)?;
        output.write_u32::<LittleEndian>(next_data_rva)?;
        next_data_rva += data_size;
    }

    for region in regions {
        trailing_data.extend_from_slice(&region.bytes[..region.bytes.len().min(u32::MAX as usize)]);
    }

    Ok((output, trailing_data, locations))
}

fn build_memory_info_list_stream(
    regions: &[MemoryRegion<'_>],
    modules: &[ModuleInfo],
    _base_rva: u32,
) -> io::Result<Vec<u8>> {
    let mut output = Vec::new();
    output.write_u32::<LittleEndian>(16)?;
    output.write_u32::<LittleEndian>(MINIDUMP_MEMORY_INFO_SIZE)?;
    output.write_u64::<LittleEndian>(regions.len() as u64)?;

    for region in regions {
        let (allocation_base, memory_type) = module_for_region(region, modules)
            .map(|module| (module.base, md::MemoryType::MEM_IMAGE.bits()))
            .unwrap_or((region.base, md::MemoryType::MEM_PRIVATE.bits()));
        let protection = permission_to_memory_protection(region.permission);

        output.write_u64::<LittleEndian>(region.base)?;
        output.write_u64::<LittleEndian>(allocation_base)?;
        output.write_u32::<LittleEndian>(protection)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u64::<LittleEndian>(region.bytes.len() as u64)?;
        output.write_u32::<LittleEndian>(md::MemoryState::MEM_COMMIT.bits())?;
        output.write_u32::<LittleEndian>(protection)?;
        output.write_u32::<LittleEndian>(memory_type)?;
        output.write_u32::<LittleEndian>(0)?;
    }

    Ok(output)
}

fn module_for_region<'a>(
    region: &MemoryRegion<'_>,
    modules: &'a [ModuleInfo],
) -> Option<&'a ModuleInfo> {
    modules
        .iter()
        .find(|module| region_belongs_to_module(region.name, &module.prefix))
}

fn build_system_info_stream(emu: &Emu, _base_rva: u32) -> io::Result<Vec<u8>> {
    let mut output = Vec::with_capacity(MINIDUMP_SYSTEM_INFO_SIZE as usize);
    let processor_architecture = match emu.cfg.arch {
        Arch::X86 => md::ProcessorArchitecture::PROCESSOR_ARCHITECTURE_INTEL as u16,
        Arch::X86_64 => md::ProcessorArchitecture::PROCESSOR_ARCHITECTURE_AMD64 as u16,
        Arch::Aarch64 => md::ProcessorArchitecture::PROCESSOR_ARCHITECTURE_ARM64 as u16,
    };
    let platform_id = match emu.os {
        OperatingSystem::Windows => md::PlatformId::VER_PLATFORM_WIN32_NT as u32,
        OperatingSystem::Linux => md::PlatformId::Linux as u32,
        OperatingSystem::MacOS => md::PlatformId::MacOs as u32,
    };
    let (major_version, minor_version, build_number, product_type) = if emu.os.is_windows() {
        (10, 0, 19041, 1)
    } else {
        (0, 0, 0, 0)
    };

    output.write_u16::<LittleEndian>(processor_architecture)?;
    output.write_u16::<LittleEndian>(6)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u8(1)?;
    output.write_u8(product_type)?;
    output.write_u32::<LittleEndian>(major_version)?;
    output.write_u32::<LittleEndian>(minor_version)?;
    output.write_u32::<LittleEndian>(build_number)?;
    output.write_u32::<LittleEndian>(platform_id)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.extend_from_slice(&[0; 24]);
    Ok(output)
}

fn build_module_list_stream(
    modules: &[ModuleInfo],
    base_rva: u32,
) -> io::Result<(Vec<u8>, Vec<u8>)> {
    let header_size = 4 + ((modules.len() as u32) * MINIDUMP_MODULE_SIZE);
    let mut next_name_rva = base_rva + header_size;
    let mut output = Vec::new();
    let mut trailing_data = Vec::new();

    output.write_u32::<LittleEndian>(modules.len() as u32)?;

    for module in modules {
        let name_blob = encode_minidump_string(&module.display_name)?;
        let name_rva = next_name_rva;
        next_name_rva += name_blob.len() as u32;

        output.write_u64::<LittleEndian>(module.base)?;
        output.write_u32::<LittleEndian>(module.size_of_image)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(name_rva)?;
        write_fixed_file_info(&mut output)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;

        trailing_data.extend_from_slice(&name_blob);
    }

    Ok((output, trailing_data))
}

fn build_thread_list_stream(
    emu: &Emu,
    regions: &[MemoryRegion<'_>],
    memory_locations: &BTreeMap<u64, MemoryLocation>,
    base_rva: u32,
) -> io::Result<(Vec<u8>, Vec<u8>)> {
    let thread_count = emu.threads.len() as u32;
    let header_size = 4 + (thread_count * MINIDUMP_THREAD_SIZE);
    let mut next_context_rva = base_rva + header_size;
    let mut entries = Vec::new();
    let mut trailing_data = Vec::new();

    for (thread_idx, thread) in emu.threads.iter().enumerate() {
        let (context, sp) = match &thread.arch {
            ArchThreadState::X86 { regs, flags, .. } => {
                let ctx = build_thread_context(ThreadContextInput::X86 {
                    arch: emu.cfg.arch,
                    regs,
                    flags,
                })?;
                let sp = match emu.cfg.arch {
                    Arch::X86 => regs.get_esp(),
                    Arch::X86_64 => regs.rsp,
                    _ => 0,
                };
                (ctx, sp)
            }
            ArchThreadState::AArch64 { regs, .. } => {
                let ctx = build_thread_context(ThreadContextInput::AArch64 { regs })?;
                (ctx, regs.sp)
            }
        };

        let context_rva = next_context_rva;
        next_context_rva += context.len() as u32;
        let (stack_base, stack_location) = stack_location_for_thread(sp, regions, memory_locations);
        let teb = if thread_idx == emu.current_thread_id {
            emu.maps
                .get_map_by_name("teb")
                .map(|teb| teb.get_base())
                .unwrap_or(0)
        } else {
            0
        };

        entries.push((
            thread,
            teb,
            stack_base,
            stack_location,
            context_rva,
            context.len() as u32,
        ));
        trailing_data.extend_from_slice(&context);
    }

    let mut output = Vec::new();
    output.write_u32::<LittleEndian>(thread_count)?;

    for (thread, teb, stack_base, stack_location, context_rva, context_size) in entries {
        output.write_u32::<LittleEndian>(thread.id.min(u32::MAX as u64) as u32)?;
        output.write_u32::<LittleEndian>(u32::from(thread.suspended))?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u32::<LittleEndian>(0)?;
        output.write_u64::<LittleEndian>(teb)?;
        output.write_u64::<LittleEndian>(stack_base)?;
        output.write_u32::<LittleEndian>(stack_location.data_size)?;
        output.write_u32::<LittleEndian>(stack_location.rva)?;
        output.write_u32::<LittleEndian>(context_size)?;
        output.write_u32::<LittleEndian>(context_rva)?;
    }

    Ok((output, trailing_data))
}

fn stack_location_for_thread(
    sp: u64,
    regions: &[MemoryRegion<'_>],
    memory_locations: &BTreeMap<u64, MemoryLocation>,
) -> (u64, MemoryLocation) {
    let Some(region) = regions
        .iter()
        .find(|region| sp >= region.base && sp < (region.base + region.bytes.len() as u64))
    else {
        return (
            0,
            MemoryLocation {
                data_size: 0,
                rva: 0,
            },
        );
    };

    let Some(location) = memory_locations.get(&region.base) else {
        return (
            0,
            MemoryLocation {
                data_size: 0,
                rva: 0,
            },
        );
    };

    (region.base, *location)
}

fn permission_to_memory_protection(permission: Permission) -> u32 {
    match permission.bits() {
        0 => md::MemoryProtection::PAGE_NOACCESS.bits(),
        1 => md::MemoryProtection::PAGE_READONLY.bits(),
        2 | 3 => md::MemoryProtection::PAGE_READWRITE.bits(),
        4 => md::MemoryProtection::PAGE_EXECUTE.bits(),
        5 => md::MemoryProtection::PAGE_EXECUTE_READ.bits(),
        6 | 7 => md::MemoryProtection::PAGE_EXECUTE_READWRITE.bits(),
        _ => md::MemoryProtection::PAGE_READWRITE.bits(),
    }
}

fn encode_minidump_string(value: &str) -> io::Result<Vec<u8>> {
    let utf16: Vec<u16> = value.encode_utf16().collect();
    let byte_len = utf16
        .len()
        .checked_mul(2)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "string too large"))?;
    let mut output = Vec::with_capacity(4 + byte_len);
    output.write_u32::<LittleEndian>(byte_len as u32)?;
    for ch in utf16 {
        output.write_u16::<LittleEndian>(ch)?;
    }
    Ok(output)
}

fn write_fixed_file_info(output: &mut Vec<u8>) -> io::Result<()> {
    output.write_u32::<LittleEndian>(md::VS_FFI_SIGNATURE)?;
    output.write_u32::<LittleEndian>(md::VS_FFI_STRUCVERSION)?;
    for _ in 0..11 {
        output.write_u32::<LittleEndian>(0)?;
    }
    Ok(())
}

fn directory_for_stream(stream: &StreamLayout) -> md::MINIDUMP_DIRECTORY {
    md::MINIDUMP_DIRECTORY {
        stream_type: stream.stream_type,
        location: md::MINIDUMP_LOCATION_DESCRIPTOR {
            data_size: stream.data.len().min(u32::MAX as usize) as u32,
            rva: stream.rva,
        },
    }
}

fn write_header(output: &mut Vec<u8>, stream_count: u32) -> io::Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .min(u32::MAX as u64) as u32;

    output.write_u32::<LittleEndian>(md::MINIDUMP_SIGNATURE)?;
    output.write_u32::<LittleEndian>(md::MINIDUMP_VERSION)?;
    output.write_u32::<LittleEndian>(stream_count)?;
    output.write_u32::<LittleEndian>(MINIDUMP_HEADER_SIZE)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(now)?;
    output.write_u64::<LittleEndian>(MINIDUMP_FLAGS_FULL_MEMORY)?;
    Ok(())
}

fn write_directory(output: &mut Vec<u8>, directory: &md::MINIDUMP_DIRECTORY) -> io::Result<()> {
    output.write_u32::<LittleEndian>(directory.stream_type)?;
    output.write_u32::<LittleEndian>(directory.location.data_size)?;
    output.write_u32::<LittleEndian>(directory.location.rva)?;
    Ok(())
}

fn pad_to_rva(output: &mut Vec<u8>, rva: u32) -> io::Result<()> {
    let target = rva as usize;
    if output.len() > target {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "stream layout overlap while building minidump",
        ));
    }
    output.resize(target, 0);
    Ok(())
}
