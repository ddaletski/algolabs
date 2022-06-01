#![crate_type = "proc-macro"]

use quote::ToTokens;
use rust_config_types::{Config, ConfigSpec, Configurable};
use syn::parse::Parser;

#[macro_use]
extern crate quote;

// TODO: create `configurable` macro for structs:
//  - get_config_spec(&self) -> ConfigSpec
//  - set_config(&mut self, map<String, Value>)
//  - get_config(&self) -> map<String, Value>
//  - get_property(&self, String) -> Value
//  - set_property(&mut self, String, Value)
//  - default_spec() -> ConfigSpec
//
//  create `prop` macro for struct fields
//  collect all and implement default_spec()

#[proc_macro_attribute]
pub fn configurable(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let mut ast: syn::ItemStruct = syn::parse2(input.clone()).unwrap();

    if let syn::Fields::Named(fields) = &mut ast.fields {
        let field_code = quote! {config_spec: ConfigSpec };
        let field_ast = syn::Field::parse_named.parse2(field_code).unwrap();
        fields.named.push(field_ast);
    }

    let struct_name = ast.ident.clone();

    let source = ast.into_token_stream();
    let code = quote! {
        #source

        impl Configurable for #struct_name {
            fn set_config(&mut self, config: Config) {
                for (k, v) in config {
                    self.config_spec.set(&k, v);
                }
            }

            fn get_config(&self) -> Config {
                self.config_spec.values()
            }
        }
    };

    code.into()
}
