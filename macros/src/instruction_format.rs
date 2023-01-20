use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{
    parse::Parser, parse_macro_input, Error, Ident, ItemEnum, ItemStruct, LitInt, Result, Token,
};

enum FromFormatInput {
    Construct(Vec<(Ident, LitInt)>),
    Rename(Ident),
}

fn parse_assign(input: ParseStream) -> Result<(Ident, LitInt)> {
    let i = Ident::parse(input)?;
    <Token![=]>::parse(input)?;
    let v = LitInt::parse(input)?;
    Ok((i, v))
}

impl Parse for FromFormatInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(checks) = Checks::parse(input) {
            Ok(FromFormatInput::Construct(checks.0))
        } else {
            let id = Ident::parse(input)?;
            Ok(FromFormatInput::Rename(id))
        }
    }
}

struct FormatInput {
    format: Ident,
}

impl Parse for FormatInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let format = Ident::parse(input)?;
        Ok(FormatInput { format })
    }
}

#[derive(Default, Clone)]
struct Checks(Vec<(Ident, LitInt)>);

impl Parse for Checks {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut checks = vec![];

        while let Ok(assign) = parse_assign(input) {
            checks.push(assign);
            if let Err(_) = <Token![,]>::parse(input) {
                break;
            }
        }

        if checks.len() == 0 {
            return Err(Error::new(
                input.span(),
                "Checks should have at least one check",
            ));
        }

        Ok(Checks(checks))
    }
}

fn gen_const_impls(ident: &Ident, args: &[(Ident, LitInt)]) -> proc_macro2::TokenStream {
    let consts = args
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

    let consts = quote!(
    impl #ident {
        #consts
    }
    );

    proc_macro2::TokenStream::from(consts)
}

fn gen_checks(vars: &[(Ident, LitInt)]) -> proc_macro2::TokenStream {
    let checks = vars
        .iter()
        .map(|(i, v)| {
            quote!(
            f.#i == #v
            )
        })
        .fold(None, |acc, ts| {
            if let Some(acc) = acc {
                Some(quote!(#acc && #ts))
            } else {
                Some(ts)
            }
        });

    proc_macro2::TokenStream::from(quote!(#checks))
}

fn gen_from_format(
    format_ident: &Ident,
    item_ident: &Ident,
    checks: proc_macro2::TokenStream,
    new_item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let try_from_format = quote!(
    impl TryFrom<#format_ident> for #item_ident {
        type Error = ();
        fn try_from(f: #format_ident) -> Result<#item_ident, Self::Error> {
            if #checks {
                Ok(#new_item)
            } else {
                Err(())
            }
        }
    }
    );

    let from_format_unsafe = quote!(
    impl #item_ident {
        pub fn from_format_unsafe(f: #format_ident) -> #item_ident {
            #new_item
        }
    }
    );

    let ts = quote!(
    #try_from_format
    #from_format_unsafe
               );

    proc_macro2::TokenStream::from(ts)
}

// #[format] is mandatory for structs
// #[checks] is mandatory for structs
fn struct_instruction_format(input: TokenStream) -> Result<TokenStream> {
    let mut strut = Parser::parse(ItemStruct::parse, input)?;
    let struct_ident = &strut.ident;

    let format_idx = strut
        .attrs
        .iter()
        .position(|attr| attr.path.is_ident("format"));

    let format = format_idx
        .map(|idx| strut.attrs.get(idx))
        .flatten()
        .map(|attr| attr.parse_args_with(FormatInput::parse));

    if let Some(idx) = format_idx {
        strut.attrs.remove(idx);
    }

    let checks_idx = strut
        .attrs
        .iter()
        .position(|attr| attr.path.is_ident("checks"));

    let global_checks = checks_idx
        .map(|idx| strut.attrs.get(idx))
        .flatten()
        .map(|attr| attr.parse_args_with(Checks::parse));

    let global_checks = match global_checks {
        Some(Ok(checks)) => checks.0,
        Some(Err(e)) => return Err(e),
        None => vec![],
    };

    if let Some(idx) = checks_idx {
        strut.attrs.remove(idx);
    }

    let args = match format {
        Some(Ok(args)) => args,
        Some(Err(err)) => return Ok(TokenStream::from(err.to_compile_error())),
        None => {
            return Ok(TokenStream::from(
                Error::new(
                    proc_macro2::Span::call_site(),
                    "Structs must provide a format attribute",
                )
                .to_compile_error(),
            ))
        }
    };

    let format_ident = &args.format;

    let struct_fields: Vec<_> = strut
        .fields
        .iter_mut()
        .map(|field| {
            let attr_idx = field
                .attrs
                .iter()
                .position(|attr| attr.path.is_ident("format_mapping"));

            let rename = attr_idx
                .map(|idx| field.attrs.get(idx))
                .flatten()
                .map(|attr| attr.parse_args_with(FromFormatInput::parse));

            if let Some(idx) = attr_idx {
                field.attrs.remove(idx);
            }

            (field, rename)
        })
        .collect();

    let consts = gen_const_impls(struct_ident, &global_checks);

    let checks = gen_checks(&global_checks);

    let from_format_struct_fields = struct_fields
        .iter()
        .map(|(f, alias)| {
            let field = &f.ident;
            if let Some(alias) = alias {
                match alias {
                    Ok(FromFormatInput::Rename(alias)) => {
                        quote!(#field: f.#alias)
                    }
                    Ok(FromFormatInput::Construct(fields)) => {
                        let assign = fields.iter().map(|(id, v)| quote!(f.#id << #v)).fold(
                            None,
                            |acc, v| {
                                if let Some(acc) = acc {
                                    Some(quote!(#acc | #v))
                                } else {
                                    Some(quote!(#v))
                                }
                            },
                        );
                        quote!(#field: #assign)
                    }
                    Err(err) => proc_macro2::TokenStream::from(err.to_compile_error()),
                }
            } else {
                quote!(
                #field: f.#field
                  )
            }
        })
        .fold(None, |acc, st| {
            if let Some(acc) = acc {
                Some(quote!(#acc, #st))
            } else {
                Some(quote!(#st))
            }
        });

    let from_format_new_struct = quote!(
    #struct_ident {
        #from_format_struct_fields
    }
    );

    let from_format = gen_from_format(format_ident, struct_ident, checks, from_format_new_struct);

    let args_format_fields =
        global_checks
            .iter()
            .map(|(i, v)| quote!(#i: #v))
            .fold(None, |acc, st| {
                if let Some(acc) = acc {
                    Some(quote!(#acc, #st))
                } else {
                    Some(quote!(#st))
                }
            });

    let strut_format_fields = struct_fields
        .iter()
        .map(|(f, alias)| {
            let field = &f.ident;
            if let Some(alias) = alias {
                match alias {
                    Ok(FromFormatInput::Rename(alias)) => {
                        quote!(#alias: i.#field)
                    }
                    Ok(FromFormatInput::Construct(fields)) => fields
                        .iter()
                        .map(|(id, v)| quote!(#id: i.#field >> #v))
                        .fold(None, |acc, v| {
                            if let Some(acc) = acc {
                                Some(quote!(#acc, #v))
                            } else {
                                Some(quote!(#v))
                            }
                        })
                        .unwrap_or(proc_macro2::TokenStream::from(
                            Error::new(proc_macro2::Span::call_site(), "Alias has no constructor")
                                .to_compile_error(),
                        )),
                    Err(err) => proc_macro2::TokenStream::from(err.to_compile_error()),
                }
            } else {
                quote!(
                #field: i.#field
                  )
            }
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

    let st = quote!(
    #strut

    #consts

    #from_format

    #into_format

    #num_conversions
               );

    Ok(TokenStream::from(st))
}

// #[format] is optional for enums
// If format is present
//      - generate TryFrom<Format>, generate checks and use variant::from_format_unsafe()
//      - generate TryFrom<u32>, using Format.
// If not present
//      - generate the TryFrom<u32> and call variant::try_from()
fn enum_instruction_format(input: TokenStream) -> TokenStream {
    let mut enu = parse_macro_input!(input as ItemEnum);

    let enum_ident = &enu.ident;

    let format_idx = enu
        .attrs
        .iter()
        .position(|attr| attr.path.is_ident("format"));

    let format = format_idx
        .map(|idx| enu.attrs.get(idx))
        .flatten()
        .map(|attr| attr.parse_args_with(FormatInput::parse));

    let format = match format {
        Some(Ok(format)) => Some(format),
        Some(Err(err)) => return TokenStream::from(err.to_compile_error()),
        None => None,
    };

    if let Some(idx) = format_idx {
        enu.attrs.remove(idx);
    }

    let checks_idx = enu
        .attrs
        .iter()
        .position(|attr| attr.path.is_ident("checks"));

    let global_checks = checks_idx
        .map(|idx| enu.attrs.get(idx))
        .flatten()
        .map(|attr| attr.parse_args_with(Checks::parse));

    let global_checks = match global_checks {
        Some(Ok(checks)) => checks.0,
        Some(Err(e)) => return TokenStream::from(e.to_compile_error()),
        None => vec![],
    };

    if let Some(idx) = checks_idx {
        enu.attrs.remove(idx);
    }

    let variant_checks: Result<_> = enu
        .variants
        .iter_mut()
        .map(|variant| {
            let check_idx = variant
                .attrs
                .iter()
                .position(|attr| attr.path.is_ident("checks"));

            let checks = check_idx
                .map(|idx| variant.attrs.get(idx))
                .flatten()
                .map(|attr| attr.parse_args_with(Checks::parse));

            if let Some(idx) = check_idx {
                variant.attrs.remove(idx);
            }

            (variant, checks)
        })
        .fold(Ok(Vec::new()), |acc, (variant, checks)| {
            if let Ok(mut vec) = acc {
                let checks = match checks {
                    Some(Ok(checks)) => Some(checks),
                    Some(Err(err)) => return Err(err),
                    None => None,
                };
                vec.push((variant, checks));
                return Ok(vec);
            } else {
                return acc;
            }
        });

    let variant_checks = match variant_checks {
        Ok(variant_checks) => variant_checks,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let variants_into = variant_checks
        .iter()
        .map(|(variant, _)| {
            let variant_ident = &variant.ident;
            quote!(#enum_ident::#variant_ident(i) => i.into(),)
        })
        .fold(quote!(), |acc, next| quote!(#acc #next));

    let to_generate = if let Some(format) = format {
        let check_count = variant_checks
            .iter()
            .filter(|(_, checks)| checks.is_some())
            .count();
        if check_count != variant_checks.len() {
            return TokenStream::from(
                Error::new(
                    proc_macro2::Span::call_site(),
                    "All variants should have a checks attribute when a format is present",
                )
                .to_compile_error(),
            );
        }

        if global_checks.len() == 0 {
            return TokenStream::from(
                Error::new(
                    proc_macro2::Span::call_site(),
                    "An enum with a format attribute should have a global attribute checks",
                )
                .to_compile_error(),
            );
        }

        let format_ident = &format.format;

        let global_checks = gen_checks(&global_checks);

        let variants_from_format = variant_checks
            .iter()
            .enumerate()
            .map(|(idx, (variant, checks))| {
                let variant_ident = &variant.ident;
                let variant_field = variant.fields.iter().nth(0).unwrap();
                let checks = gen_checks(&checks.clone().unwrap_or_default().0);
                if idx == 0 {
                    quote!(
                        if #checks {
                            return Ok(#enum_ident::#variant_ident(#variant_field::from_format_unsafe(f)));
                        }
                        )
                } else {
                    quote!(
                        else if #checks {
                            return Ok(#enum_ident::#variant_ident(#variant_field::from_format_unsafe(f)));
                        }
                        )
                }
            })
        .fold(quote!(), |acc, next| quote!(#acc #next));

        let from_format = quote!(
        impl TryFrom<#format_ident> for #enum_ident {
            type Error = ();
            fn try_from(f: #format_ident) -> Result<#enum_ident, Self::Error> {
                if #global_checks {
                    #variants_from_format
                }
                Err(())
            }
        }
        );

        let from_u32 = quote!(
        impl TryFrom<u32> for #enum_ident {
            type Error = ();
            fn try_from(n: u32) -> Result<#enum_ident, Self::Error> {
                let f = #format_ident::from(n);
                #enum_ident::try_from(f)
            }
        }
        );

        let into_format = quote!(
        impl From<#enum_ident> for #format_ident {
            fn from(i: #enum_ident) -> #format_ident {
                match i {
                    #variants_into
                }
            }
        }
        );

        let into_u32 = quote!(
        impl From<#enum_ident> for u32 {
            fn from(i: #enum_ident) -> u32 {
                let f = #format_ident::from(i);
                f.into()
            }
        }
        );

        quote!(
        #from_format
        #from_u32
        #into_format
        #into_u32
          )
    } else {
        let variants_from_u32 = enu
            .variants
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let variant_ident = &variant.ident;
                let variant_value = variant.fields.iter().nth(0).unwrap();
                if idx == 0 {
                    quote!(
                        if let Ok(i) = #variant_value::try_from(n) {
                            return Ok(#enum_ident::#variant_ident(i))
                        }
                    )
                } else {
                    quote!(
                        else if let Ok(i) = #variant_value::try_from(n) {
                            return Ok(#enum_ident::#variant_ident(i))
                        }
                    )
                }
            })
            .fold(quote!(), |acc, next| quote!(#acc #next));

        let from_u32 = quote!(
            impl TryFrom<u32> for #enum_ident {
                type Error = ();
                fn try_from(n: u32) -> Result<#enum_ident, Self::Error> {
                    #variants_from_u32
                    Err(())
                }
            }
        );

        let into_u32 = quote!(
            impl From<#enum_ident> for u32 {
                fn from(i: #enum_ident) -> u32 {
                    match i {
                        #variants_into
                    }
                }
            }
        );

        quote!(
            #from_u32
            #into_u32
        )
    };

    let consts = gen_const_impls(enum_ident, &global_checks);

    let st = quote!(
    #enu
    #consts
    #to_generate
               );

    TokenStream::from(st)
}

pub fn instruction(_args: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(st) = struct_instruction_format(input.clone()) {
        st
    } else {
        enum_instruction_format(input)
    }
}
