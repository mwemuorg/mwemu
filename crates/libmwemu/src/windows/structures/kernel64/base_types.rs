use crate::maps::Maps;

#[derive(Debug)]
pub struct KSystemTime {
    pub low_part: u32,
    pub high1_time: u32,
    pub high2_time: u32,
}

impl Default for KSystemTime {
    fn default() -> Self {
        Self::new()
    }
}

impl KSystemTime {
    pub fn size() -> u32 {
        12
    }

    pub fn new() -> KSystemTime {
        KSystemTime {
            low_part: 0,
            high1_time: 0,
            high2_time: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> KSystemTime {
        KSystemTime {
            low_part: maps.read_dword(addr).unwrap(),
            high1_time: maps.read_dword(addr + 4).unwrap(),
            high2_time: maps.read_dword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.low_part);
        maps.write_dword(addr + 4, self.high1_time);
        maps.write_dword(addr + 8, self.high2_time);
    }
}

#[derive(Debug)]
pub struct SSDT {
    pub p_service_table: u64,
    pub p_counter_table: u64,
    pub number_of_services: u32,
    pub p_argument_table: u64,
}

impl Default for SSDT {
    fn default() -> Self {
        Self::new()
    }
}

impl SSDT {
    pub fn size() -> u32 {
        28
    }

    pub fn new() -> SSDT {
        SSDT {
            p_service_table: 0,
            p_counter_table: 0,
            number_of_services: 0,
            p_argument_table: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SSDT {
        SSDT {
            p_service_table: maps.read_qword(addr).unwrap(),
            p_counter_table: maps.read_qword(addr + 8).unwrap(),
            number_of_services: maps.read_dword(addr + 16).unwrap(),
            p_argument_table: maps.read_qword(addr + 20).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.p_service_table);
        maps.write_qword(addr + 8, self.p_counter_table);
        maps.write_dword(addr + 16, self.number_of_services);
        maps.write_qword(addr + 20, self.p_argument_table);
    }
}

#[derive(Debug)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: u64,
}

impl Default for UnicodeString {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeString {
    pub fn size() -> u32 {
        12
    }

    pub fn new() -> UnicodeString {
        UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString {
        UnicodeString {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            buffer: maps.read_qword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_qword(addr + 4, self.buffer);
    }
}

#[derive(Debug, Clone)]
pub struct DeviceIoControl {
    pub output_buffer_length: u32,
    pub input_buffer_length: u32,
    pub io_control_code: u32,
    pub type3_input_buffer: u64,
}

impl Default for DeviceIoControl {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceIoControl {
    pub fn size() -> u32 {
        20
    }

    pub fn new() -> DeviceIoControl {
        DeviceIoControl {
            output_buffer_length: 0,
            input_buffer_length: 0,
            io_control_code: 0,
            type3_input_buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> DeviceIoControl {
        DeviceIoControl {
            output_buffer_length: maps.read_dword(addr).unwrap(),
            input_buffer_length: maps.read_dword(addr + 4).unwrap(),
            io_control_code: maps.read_dword(addr + 8).unwrap(),
            type3_input_buffer: maps.read_qword(addr + 12).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.output_buffer_length);
        maps.write_dword(addr + 4, self.input_buffer_length);
        maps.write_dword(addr + 8, self.io_control_code);
        maps.write_qword(addr + 12, self.type3_input_buffer);
    }
}

#[derive(Debug)]
pub struct StringStruct {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: u64,
}

impl Default for StringStruct {
    fn default() -> Self {
        Self::new()
    }
}

impl StringStruct {
    pub fn size() -> u32 {
        12
    }

    pub fn new() -> StringStruct {
        StringStruct {
            length: 0,
            maximum_length: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> StringStruct {
        StringStruct {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            buffer: maps.read_qword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_qword(addr + 4, self.buffer);
    }
}

#[derive(Debug)]
pub struct ClientId {
    pub unique_process: u32,
    pub unique_thread: u32,
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientId {
    pub fn size() -> u32 {
        8
    }

    pub fn new() -> ClientId {
        ClientId {
            unique_process: 0,
            unique_thread: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> ClientId {
        ClientId {
            unique_process: maps.read_dword(addr).unwrap(),
            unique_thread: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.unique_process);
        maps.write_dword(addr + 4, self.unique_thread);
    }
}

#[derive(Debug)]
pub struct LargeInteger {
    pub low_part: u32,
    pub high_part: u32,
}

impl Default for LargeInteger {
    fn default() -> Self {
        Self::new()
    }
}

impl LargeInteger {
    pub fn new() -> LargeInteger {
        LargeInteger {
            low_part: 0,
            high_part: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> LargeInteger {
        LargeInteger {
            low_part: maps.read_dword(addr).unwrap(),
            high_part: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.low_part);
        maps.write_dword(addr + 4, self.high_part);
    }
}
