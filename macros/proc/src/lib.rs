use proc_macro::TokenStream;
use quote::quote;
use syn::{Lit, LitInt, Token, parse::Parse, parse_macro_input};

struct RunYearArgs {
    year: String,
    last_day: LitInt,
}

impl Parse for RunYearArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year_lit = input.parse()?;
        let year = match year_lit {
            Lit::Int(i) => i.base10_digits().to_owned(),
            _ => return Err(syn::Error::new(year_lit.span(), "Expected integer for year")),
        };

        input.parse::<Token![,]>()?;

        let day_lit = input.parse()?;
        let Lit::Int(last_day) = day_lit else {
            return Err(syn::Error::new(day_lit.span(), "Expected integer for last day"));
        };
        if !last_day.suffix().is_empty() {
            return Err(syn::Error::new(last_day.span(), format!("Expected unsuffixed integer for last day, got {}", last_day)));
        }

        Ok(RunYearArgs { year, last_day })
    }
}

#[proc_macro]
pub fn run_year(input: TokenStream) -> TokenStream {
    let RunYearArgs{
        year,
        last_day
    } = parse_macro_input!(input as RunYearArgs);
    
    let module_ident = syn::Ident::new(&format!("year{}", year), proc_macro2::Span::call_site());

    let expanded = quote! {
        {
            use rules::*;
            use crate::years::#module_ident::*;
            use seq_macro::seq;

            seq!(D in 01..=#last_day {
                let input = include_bytes!(concat!(
                    "../../input/",
                    #year, 
                    "/",
                    stringify!(D), 
                    "/input.txt"
                ));
                let parsed = parse_expect!(D, input);
                _run_day_part_preparsed!(D, 1, parsed);
                _run_day_part_preparsed!(D, 2, parsed);
            });
        }
    };

    TokenStream::from(expanded)
}
