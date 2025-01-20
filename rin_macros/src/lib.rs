extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Metrics)]
pub fn print_metrics_derive(input: TokenStream) -> TokenStream {
    // Parse the input as a Rust struct
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Metrics can only be used on structs"),
    };

    // Create the code to collect metrics data for the fields ending in "_metric"
    let field_data = if let Fields::Named(fields) = fields {
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
                                    data.push((#field_name_str.to_string(), format!("{:?}", value)));
                                }
                            })
                        } else {
                            // Non-Option type handling
                            Some(quote! {
                                data.push((#field_name_str.to_string(), format!("{:?}", self.#field_name)));
                            })
                        }
                    } else {
                        // Non-Option type handling
                        Some(quote! {
                            data.push((#field_name_str.to_string(), format!("{:?}", self.#field_name)));
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

    // Generate the code for the Metrics trait and implementation for the struct
    let expanded = quote! {
        impl Metric for #struct_name {
            fn collect_metrics(&self) -> Option<Vec<(String, String)>> {
                let mut data = Vec::new();
                #(#field_data)*
                Some(data)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Messages)]
pub fn message_derive(input: TokenStream) -> TokenStream {
    // Parse the input as a Rust struct
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    // Generate the code to implement the Message trait for the struct
    let expanded = quote! {
        impl Message for #struct_name {}
    };

    // Return the generated implementation as a TokenStream
    TokenStream::from(expanded)
}
