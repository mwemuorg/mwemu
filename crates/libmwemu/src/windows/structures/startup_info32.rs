use crate::maps::Maps;

#[derive(Debug)]
pub struct StartupInfo32 {
    cb: u32,
    reserved: u32,
    desktop: u32,
    title: u32,
    x: u32,
    y: u32,
    x_size: u32,
    y_size: u32,
    x_count_chars: u32,
    y_count_chars: u32,
    fill_attribute: u32,
    flags: u32,
    show_window: u16,
    cb_reserved2: u16,
    lp_reserved2: u32,
    std_input: u32,
    std_output: u32,
    std_error: u32,
}

impl Default for StartupInfo32 {
    fn default() -> Self {
        Self::new()
    }
}

impl StartupInfo32 {
    pub fn new() -> StartupInfo32 {
        StartupInfo32 {
            cb: 68,
            reserved: 0,
            desktop: 0,
            title: 0,
            x: 10,
            y: 10,
            x_size: 300,
            y_size: 200,
            x_count_chars: 0,
            y_count_chars: 0,
            fill_attribute: 0,
            flags: 0,
            show_window: 1,
            cb_reserved2: 0,
            lp_reserved2: 0,
            std_input: 0,
            std_output: 0,
            std_error: 0,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.cb);
        maps.write_dword(addr + 4, self.reserved);
        maps.write_dword(addr + 8, self.desktop);
        maps.write_dword(addr + 12, self.title);
        maps.write_dword(addr + 16, self.x);
        maps.write_dword(addr + 20, self.y);
        maps.write_dword(addr + 24, self.x_size);
        maps.write_dword(addr + 28, self.y_size);
        maps.write_dword(addr + 32, self.x_count_chars);
        maps.write_dword(addr + 36, self.y_count_chars);
        maps.write_dword(addr + 40, self.fill_attribute);
        maps.write_dword(addr + 44, self.flags);
        maps.write_word(addr + 48, self.show_window);
        maps.write_word(addr + 50, self.cb_reserved2);
        maps.write_dword(addr + 52, self.lp_reserved2);
        maps.write_dword(addr + 56, self.std_input);
        maps.write_dword(addr + 60, self.std_output);
        maps.write_dword(addr + 64, self.std_error);
    }
}
