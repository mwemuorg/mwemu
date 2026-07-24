use super::Emu;

macro_rules! round_to {
    ($num:expr, $dec:expr) => {{
        let factor = 10f64.powi($dec);
        ($num * factor).round() / factor
    }};
}

impl Emu {
    pub fn update_entropy(&mut self) {
        let prev_entropy = self.entropy;

        let mem = match self.maps.get_mem_by_addr(self.pc()) {
            Some(n) => n,
            None => {
                self.entropy = 0.0;
                if self.entropy != prev_entropy {
                    log::trace!(
                        "{}:0x{:x} entropy changed {} ->  {}",
                        self.pos,
                        self.pc(),
                        prev_entropy,
                        self.entropy
                    );
                }
                return;
            }
        };

        let data = mem.get_bytes();

        if data.is_empty() {
            self.entropy = 0.0;
            if self.entropy != prev_entropy {
                log::trace!(
                    "{}:0x{:x} entropy changed {} ->  {}",
                    self.pos,
                    self.pc(),
                    prev_entropy,
                    self.entropy
                );
            }
            return;
        }

        let mut counts = [0usize; 256];
        for &b in data {
            counts[b as usize] += 1;
        }
        let len = data.len() as f64;
        self.entropy = round_to!(
            counts
                .iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / len;
                    -p * p.log2()
                })
                .sum::<f64>(),
            3
        );

        if self.entropy != prev_entropy {
            log::trace!(
                "{}:0x{:x} entropy changed {} ->  {}",
                self.pos,
                self.pc(),
                prev_entropy,
                self.entropy
            );
        }
    }
}
