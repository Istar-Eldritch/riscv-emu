pub mod cpu;
pub mod emulator;
pub mod instructions;
pub mod mcu;
pub mod memory;
pub mod peripherals;
pub mod terminal;
pub mod utils;

#[cfg(test)]
mod tests {
    use macros::mask;
    #[test]
    fn mask_macro_works() {
        let m = mask!(3);
        assert_eq!(m, 0b111);
    }
}
