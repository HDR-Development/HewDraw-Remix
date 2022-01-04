use std::path::Path;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{__private::ToTokens, spanned::Spanned};

use std::fmt::Write;

#[proc_macro]
pub fn size_of_rom_file(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(TokenStream2::from(item), format!("Failed to parse input as string literal: {:?}", e)).into_compile_error().into();
        }
    };
    let local_path = literal.value();

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.").into_compile_error().into()
        }
    };

    let complete_path = parent.join("../../romfs/build/").join(local_path);
    match std::fs::metadata(&complete_path) {
        Ok(metadata) => syn::LitInt::new(format!("{}", metadata.len()).as_str(), literal.span()).to_token_stream().into(),
        Err(e) => syn::Error::new(literal.span(), format!("Could not get file size of path {}: {:?}", complete_path.display(), e)).into_compile_error().into()
    }
}

#[proc_macro]
pub fn rom_path(item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(item.span(), "Failed to get parent of current path.").into_compile_error().into()
        }
    };

    let full_path = parent.join("../../romfs/build/");

    let full_path = match full_path.as_os_str().to_str() {
        Some(path) => path,
        None => return syn::Error::new(item.span(), "Full path contains invalid characters!").into_compile_error().into()
    };
    
    syn::LitStr::new(full_path, item.span()).to_token_stream().into()
}

#[proc_macro]
pub fn from_root(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(TokenStream2::from(item), format!("Failed to parse input as string literal: {:?}", e)).into_compile_error().into();
        }
    };

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.").into_compile_error().into()
        }
    };

    let full_path = parent.join("../../").join(literal.value());

    let full_path = match full_path.as_os_str().to_str() {
        Some(path) => path,
        None => return syn::Error::new(literal.span(), "Full path contains invalid characters!").into_compile_error().into()
    };
    
    syn::LitStr::new(full_path, literal.span()).to_token_stream().into()
}

#[proc_macro]
pub fn agent_params(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(TokenStream2::from(item), format!("Failed to parse input as string literal: {:?}", e)).into_compile_error().into();
        }
    };

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.").into_compile_error().into()
        }
    };

    let full_path = parent.join("../../").join(literal.value());

    let data = match std::fs::read_to_string(&full_path) {
        Ok(result) => result,
        Err(e) => {
            return syn::Error::new(literal.span(), format!("Failed to open file at '{}': {:?}", full_path.display(), e)).into_compile_error().into();
        }
    };

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.").into_compile_error().into()
        }
    };

    let rom_path = parent.join("../../romfs/build/");
    
    let mut output = String::new();
    for line in data.lines() {
        let line = line.trim_start();
        if !line.starts_with("#") {
            if let Some((agent, file)) = line.split_once(":") {
                let agent = agent.trim_start().trim_end();
                let file = file.trim_start().trim_end();
                if let Ok(metadata) = std::fs::metadata(rom_path.join(file)) {
                    writeln!(output, "{}:{}:{}", agent, file, metadata.len()).expect("Unknown error writing to output!");
                }
            }
        }
    }
    syn::LitStr::new(output.as_str(), literal.span()).to_token_stream().into()
}