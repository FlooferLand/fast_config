use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

#[proc_macro_derive(FastConfig)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let crate_path = quote! {fast_config};
    let path_type = quote! {impl AsRef<std::path::Path>};

    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    quote! {
        impl #crate_path::Config for #ident {
            fn load(&mut self, path: #path_type, format: #crate_path::Format ) -> Result<(), #crate_path::Error> {
                let mut content = String::new();
                let mut file = std::fs::File::open(path)?;
                std::io::Read::read_to_string(&mut file, &mut content)?;
                *self = #crate_path::Config::from_string(&content, format)?;
                Ok(())
            }
            fn save(&self, path: #path_type, format: #crate_path::Format ) -> Result<(), #crate_path::Error> {
                if let Some(parent_dir) = path.as_ref().parent() {
                    std::fs::create_dir_all(parent_dir)?;
                }
                let mut file = std::fs::File::create(path)?;
                let content = #crate_path::Config::to_string(self, format)?;
                use std::io::Write;
                write!(file, "{}", content)?;
                Ok(())
            }
            fn save_pretty(&self, path: #path_type, format: #crate_path::Format ) -> Result<(), #crate_path::Error> {
                if let Some(parent_dir) = path.as_ref().parent() {
                    std::fs::create_dir_all(parent_dir)?;
                }
                let mut file = std::fs::File::create(path)?;
                let content = #crate_path::Config::to_string_pretty(self, format)?;
                use std::io::Write;
                write!(file, "{}", content)?;
                Ok(())
            }
        }
    }.into()
}