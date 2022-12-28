mod cpu;
mod emulator;
mod instructions;
mod mcu;
mod memory;
mod terminal;
mod utils;

pub use emulator::{Emulator, EmulatorOpts};
pub use terminal::TermEmulator;

#[cfg(test)]
mod tests {
    use macros::mask;
    #[test]
    fn mask_macro_works() {
        let m = mask!(3);
        assert_eq!(m, 0b111);
    }
}
