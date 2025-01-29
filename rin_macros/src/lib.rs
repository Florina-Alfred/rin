extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Meta, MetaList, Path};

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

// #[proc_macro_derive(Messages, attributes(serde))]
// pub fn message_derive(input: TokenStream) -> TokenStream {
//     // Parse the input token stream into a syntax tree
//     let input = parse_macro_input!(input as DeriveInput);
//
//     // Get the struct name and existing attributes
//     let name = &input.ident;
//     let mut existing_serde = false;
//
//     // Check for existing `Serialize` and `Deserialize` derives
//     for attr in &input.attrs {
//         // Check if the attribute is a `derive` list
//         if let Ok(Meta::List(MetaList { path, nested, .. })) = attr.parse_meta() {
//             if path.is_ident("derive") {
//                 // Check each nested meta item to see if it's `Serialize` or `Deserialize`
//                 for meta in nested {
//                     if let Meta::Path(ref path) = meta {
//                         if path.is_ident("Serialize") || path.is_ident("Deserialize") {
//                             existing_serde = true;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     // Create a new derive list that includes `Messages` and `Serialize`/`Deserialize` if needed
//     let mut derive_tokens = vec!["Messages"];
//     if !existing_serde {
//         derive_tokens.push("Serialize");
//         derive_tokens.push("Deserialize");
//     }
//
//     // Generate the new derive attribute with added `Serialize` and `Deserialize` if necessary
//     let derive_tokens = derive_tokens
//         .iter()
//         .map(|s| syn::Ident::new(*s, proc_macro2::Span::call_site()));
//
//     // Generate the impl block for the Message trait
//     let expanded = quote! {
//         #[derive(#(#derive_tokens),*)]
//         #input
//
//         impl Message for #name {
//             async fn next(&mut self) -> Option<&mut Self> {
//                 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//                 Some(self)
//             }
//
//             fn ser(&self) -> String {
//                 serde_json::to_string(&self).unwrap()
//             }
//
//             fn deser(&self, msg: &String) -> Self {
//                 serde_json::from_str(&msg).unwrap()
//             }
//         }
//     };
//
//     // Convert the generated code back into a TokenStream
//     TokenStream::from(expanded)
// }

#[proc_macro_derive(Messages)]
pub fn message_derive(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let name = &input.ident;

    // Generate the impl block for the Message trait
    let expanded = quote! {
        impl Message for #name {
            async fn next(&mut self) -> Option<&mut Self> {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                Some(self)
            }

            fn ser(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }

            fn deser(&self, msg: &String) -> Self {
                serde_json::from_str(&msg).unwrap()
            }
        }
    };

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}
