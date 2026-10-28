//! The memory surface a Spectrum-family ULA reads the screen through.
//!
//! Source references:
//! - `knowledge/systems/spectrum/overview.md`
//! - `knowledge/systems/spectrum/contention.md`

/// Spectrum-family memory surface used by the machine and ULA layers.
pub trait MemoryBus {
    /// Reads one byte from the machine address space.
    fn read(&self, addr: u16) -> u8;

    /// Writes one byte to the machine address space.
    ///
    /// ROM writes are silently ignored.
    fn write(&mut self, addr: u16, value: u8);

    /// Returns `true` if the address lies in contended RAM.
    fn is_contended(&self, addr: u16) -> bool;

    /// Reads one byte from the ULA-visible screen bank.
    ///
    /// On the 48K machine this is identical to `read`.
    fn read_screen(&self, addr: u16) -> u8 {
        self.read(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A flat 64 KB space whose bottom 16 KB is ROM, with only the
    /// required methods implemented.
    struct FlatMemory {
        bytes: [u8; 0x10000],
    }

    impl MemoryBus for FlatMemory {
        fn read(&self, addr: u16) -> u8 {
            self.bytes[usize::from(addr)]
        }

        fn write(&mut self, addr: u16, value: u8) {
            if addr >= 0x4000 {
                self.bytes[usize::from(addr)] = value;
            }
        }

        fn is_contended(&self, addr: u16) -> bool {
            (0x4000..0x8000).contains(&addr)
        }
    }

    #[test]
    fn screen_reads_default_to_ordinary_reads() {
        let mut memory = FlatMemory {
            bytes: [0; 0x10000],
        };
        memory.write(0x4000, 0xA5);
        memory.write(0x0000, 0x5A);

        assert_eq!(memory.read_screen(0x4000), 0xA5);
        assert_eq!(memory.read_screen(0x0000), memory.read(0x0000));
        assert_eq!(memory.read(0x0000), 0, "ROM writes are ignored");
        assert!(memory.is_contended(0x4000));
        assert!(!memory.is_contended(0x8000));
    }
}
