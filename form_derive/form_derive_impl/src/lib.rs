use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Ident};

fn snake_to_pascal(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for c in input.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[proc_macro_derive(Form)]
pub fn derive_form(input: TokenStream) -> TokenStream {
    // do something with the token stream here
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            let pascal_case_vals = fields.named.iter().filter_map(|field| {
                let name = field.ident.clone()?.to_string();
                let f = snake_to_pascal(&name);
                let ident = Ident::new(&f, Span::call_site().into());
                quote!(#ident).into()
            });

            let vals = pascal_case_vals.clone();

            let snake_case_vals = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!(#name)
            });

            let enum_name = Ident::new(&format!("{}Fields", input.ident), Span::call_site().into());
            let struct_name = input.ident;

            return TokenStream::from(quote!(
                #[derive(Clone, Copy)]
                pub enum #enum_name {
                    #(#pascal_case_vals),*
                }

                impl std::fmt::Display for #enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(
                                #enum_name::#vals => write!(f, stringify!(#snake_case_vals)),
                            )*
                        }
                    }
                }

                impl form_derive::FormFieldValues<#struct_name> for #enum_name {}
                impl form_derive::Form for #struct_name {}
            ));
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `FromRow`",
        )
        .to_compile_error(),
    )
}
