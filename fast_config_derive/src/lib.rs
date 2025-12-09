use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;
use syn::Attribute;
use syn::Meta;

#[proc_macro_derive(FastConfig, attributes(fast_config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let path_type = quote! {impl AsRef<std::path::Path>};

    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    
    // Extract crate path from attributes, default to "fast_config"
    let crate_path = extract_crate_path(&input.attrs);

    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let where_clause = quote! { where
        Self: for<'a> Deserialize<'a> + Serialize + Sized
    };
    quote! {
        impl #impl_generics #crate_path::FastConfig for #ident #ty_generics #where_clause {
            fn load(&mut self, path: #path_type, format: #crate_path::Format) -> Result<(), #crate_path::Error> {
                let mut content = String::new();
                let mut file = std::fs::File::open(path)?;
                std::io::Read::read_to_string(&mut file, &mut content)?;
                *self = #crate_path::FastConfig::from_string(&content, format)?;
                Ok(())
            }
            fn save(&self, path: #path_type, format: #crate_path::Format) -> Result<(), #crate_path::Error> {
                if let Some(parent_dir) = path.as_ref().parent() {
                    std::fs::create_dir_all(parent_dir)?;
                }
                let mut file = std::fs::File::create(path)?;
                let content = #crate_path::FastConfig::to_string(self, format)?;
                use std::io::Write;
                write!(file, "{}", content)?;
                Ok(())
            }
            fn save_pretty(&self, path: #path_type, format: #crate_path::Format) -> Result<(), #crate_path::Error> {
                if let Some(parent_dir) = path.as_ref().parent() {
                    std::fs::create_dir_all(parent_dir)?;
                }
                let mut file = std::fs::File::create(path)?;
                let content = #crate_path::FastConfig::to_string_pretty(self, format)?;
                use std::io::Write;
                write!(file, "{}", content)?;
                Ok(())
            }
        }
    }.into()
}

fn extract_crate_path(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    for attr in attrs {
        if attr.path().is_ident("fast_config") {
            // Parse the attribute content - for #[fast_config(crate = "...")]
            // parse_args parses what's inside the parentheses
            if let Ok(meta) = attr.parse_args::<Meta>() {
                if let Meta::NameValue(name_value) = meta {
                    if name_value.path.is_ident("crate") {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = name_value.value
                        {
                            let path_str = lit_str.value();
                            return syn::parse_str::<proc_macro2::TokenStream>(&path_str)
                                .unwrap_or_else(|_| quote! { fast_config });
                        }
                    }
                }
            }
        }
    }
    quote! { fast_config }
}
