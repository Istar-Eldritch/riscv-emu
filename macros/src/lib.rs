mod instruction_format;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitInt, Result};

struct MaskInput(LitInt);

impl Parse for MaskInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MaskInput(input.parse()?))
    }
}

#[proc_macro]
pub fn mask(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as MaskInput);
    let n: u32 = args.0.base10_digits().parse().unwrap();
    let mut mask: u32 = 0;
    for _ in 0..n {
        mask = mask << 1;
        mask = mask + 1;
    }
    let generated = quote!(#mask);
    TokenStream::from(generated)
}

#[proc_macro_attribute]
pub fn instruction(args: TokenStream, input: TokenStream) -> TokenStream {
    instruction_format::instruction(args, input)
}
