use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, ItemStruct, LitInt, Result, Token};

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

struct InstructionInput {
    format: Ident,
    vars: Vec<(Ident, LitInt)>,
}

impl Parse for InstructionInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let format = Ident::parse(input)?;
        let mut vars = vec![];
        while let Ok(_) = <Token![,]>::parse(input) {
            let i = Ident::parse(input)?;
            <Token![=]>::parse(input)?;
            let v = LitInt::parse(input)?;
            vars.push((i, v));
        }
        Ok(InstructionInput { format, vars })
    }
}

#[proc_macro_attribute]
pub fn instruction(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as InstructionInput);
    let strut = parse_macro_input!(input as ItemStruct);

    let consts = args
        .vars
        .iter()
        .map(|(i, v)| {
            let id = Ident::new(&i.to_string().to_uppercase(), Span::call_site().into());
            quote!(const #id: u32 = #v;)
        })
        .map(|ts| ts.into_token_stream())
        .fold(
            None,
            |acc: Option<proc_macro2::TokenStream>, n: proc_macro2::TokenStream| {
                if let Some(mut acc) = acc {
                    acc.extend(n);
                    Some(acc)
                } else {
                    Some(n)
                }
            },
        );

    let struct_ident = &strut.ident;
    let consts = quote!(
        impl #struct_ident {
            #consts
        }
    );

    let format_ident = &args.format;

    let checks = args
        .vars
        .iter()
        .map(|(i, _v)| {
            let i = &i;
            let iup = Ident::new(&i.to_string().to_uppercase(), Span::call_site().into());
            quote!(
                f.#i == #struct_ident::#iup
            )
        })
        .fold(None, |acc, ts| {
            if let Some(acc) = acc {
                Some(quote!(#acc && #ts))
            } else {
                Some(ts)
            }
        });

    let struct_fields = strut
        .fields
        .iter()
        .map(|f| {
            let field = &f.ident;
            quote!(
                #field: f.#field
            )
        })
        .fold(None, |acc, st| {
            if let Some(acc) = acc {
                Some(quote!(#acc, #st))
            } else {
                Some(quote!(#st))
            }
        });
    let struct_init = quote!(
        #struct_ident {
           #struct_fields
        }
    );

    let from_format = quote!(
        impl TryFrom<#format_ident> for #struct_ident {
            type Error = ();
            fn try_from(f: #format_ident) -> Result<#struct_ident, Self::Error> {
                if #checks {
                    Ok(#struct_init)
                } else {
                    Err(())
                }
            }
        }
    );

    let args_format_fields = args
        .vars
        .iter()
        .map(|(i, v)| quote!(#i: #v))
        .fold(None, |acc, st| {
            if let Some(acc) = acc {
                Some(quote!(#acc, #st))
            } else {
                Some(quote!(#st))
            }
        });

    let strut_format_fields = strut
        .fields
        .iter()
        .map(|f| {
            let id = &f.ident;
            quote!(#id: i.#id)
        })
        .fold(None, |acc, st| {
            if let Some(acc) = acc {
                Some(quote!(#acc, #st))
            } else {
                Some(quote!(#st))
            }
        });

    let into_format = quote!(
        impl From<#struct_ident> for #format_ident {
            fn from(i: #struct_ident) -> #format_ident {
                #format_ident {
                    #args_format_fields,
                    #strut_format_fields,
                    ..#format_ident::default()
                }
            }
        }
    );

    let num_conversions = quote!(
        impl TryFrom<u32> for #struct_ident {
            type Error = ();
            fn try_from(n: u32) -> Result<#struct_ident, Self::Error> {
                #struct_ident::try_from(#format_ident::from(n))
            }
        }

        impl From<#struct_ident> for u32 {
            fn from(i: #struct_ident) -> u32 {
                #format_ident::from(i).into()
            }
        }
    );

    let consts = quote!(
        #strut

        #consts

        #from_format

        #into_format

        #num_conversions
    );

    let tokenstream: TokenStream = consts.into();
    tokenstream
}
