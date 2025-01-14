extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(PrintMetrics)]
pub fn print_metrics_derive(input: TokenStream) -> TokenStream {
    // Parse the input as a Rust struct
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("PrintMetrics can only be used on structs"),
    };

    // Create the code to print metrics for the fields ending in "_metric"
    let field_prints = if let Fields::Named(fields) = fields {
        fields.named.iter().filter_map(|field| {
            if let Some(field_name) = &field.ident {
                if field_name.to_string().ends_with("_metric") {
                    let field_name_str = field_name.to_string();

                    // Check if the field is an Option type and generate code accordingly
                    if let syn::Type::Path(ref type_path) = &field.ty {
                        if type_path.path.segments.last().unwrap().ident == "Option" {
                            // Option handling
                            Some(quote! {
                                if let Some(ref value) = &self.#field_name {
                                    println!("{}: {:?}", #field_name_str, value);
                                }
                            })
                        } else {
                            // Non-Option type handling
                            Some(quote! {
                                println!("{}: {:?}", #field_name_str, self.#field_name);
                            })
                        }
                    } else {
                        // Non-Option type handling
                        Some(quote! {
                            println!("{}: {:?}", #field_name_str, self.#field_name);
                        })
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    } else {
        panic!("Named fields are required")
    };

    // Generate the function for printing
    let expanded = quote! {
        impl #struct_name {
            pub fn print_metrics(&self) {
                #(#field_prints)*
            }
        }
    };

    TokenStream::from(expanded)
}

