//! Terminal prompts, and the way into the interactive console.
//!
//! The console's command loop is in the debugger layer (`mwemu-debug`);
//! [`Console::spawn_console`] reaches it through `crate::layers`.

use crate::emu::Emu;
use std::io::Write;
use std::num::ParseIntError;

pub struct Console {}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    pub fn new() -> Console {
        log::trace!("--- console ---");
        Console {}
    }

    pub fn print(&self, msg: &str) {
        log::logger().flush();
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
    }

    pub fn cmd(&self) -> String {
        let mut line = String::new();
        self.print("=>");
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace("\r", ""); // some shells (windows) also add \r  thanks Alberto Segura
        line.truncate(line.len() - 1);
        line.to_lowercase()
    }

    pub fn cmd2(&self) -> String {
        let mut line = String::new();
        self.print("=>");
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace("\r", ""); // some shells (windows) also add \r  thanks Alberto Segura
        line.truncate(line.len() - 1);
        line
    }

    pub fn cmd_hex32(&self) -> Result<u32, ParseIntError> {
        let mut x = self.cmd();

        if x.ends_with('h') {
            x = x[0..x.len() - 1].to_string();
        }
        if x.starts_with("0x") {
            x = x[2..x.len()].to_string();
        }

        u32::from_str_radix(x.as_str(), 16)
    }

    pub fn cmd_hex64(&self) -> Result<u64, ParseIntError> {
        let mut x = self.cmd();
        if x.ends_with('h') {
            x = x[0..x.len() - 1].to_string();
        }
        if x.starts_with("0x") {
            x = x[2..x.len()].to_string();
        }

        u64::from_str_radix(x.as_str(), 16)
    }

    pub fn cmd_num(&self) -> Result<u64, ParseIntError> {
        self.cmd().as_str().parse::<u64>()
    }

    /*
    pub fn cmd_num<T>(&self) -> Result<T,ParseIntError> {
        self.cmd().as_str().parse::<T>()
    }*/

    /// Open the interactive console on the current state of `emu`.
    pub fn spawn_console(emu: &mut Emu) {
        crate::layers::debugger().spawn_console(emu);
    }
}
