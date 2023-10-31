#![feature(box_into_inner)]
use std::path::Path;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{__private::ToTokens, parse_macro_input, spanned::Spanned};

use std::fmt::Write;

#[proc_macro_attribute]
pub fn opff(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as syn::Ident);
    let usr_fn = parse_macro_input!(item as syn::ItemFn);

    let usr_fn_name = usr_fn.sig.ident.clone();

    let runtime_name = quote::format_ident!("{}_runtime", usr_fn_name);
    let static_name = quote::format_ident!("{}_static", usr_fn_name);

    quote::quote!(
        #usr_fn

        #[smashline::fighter_frame(agent = #attrs, main)]
        fn #static_name(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
            #[allow(unused_unsafe)]
            unsafe {
                #usr_fn_name(fighter)
            }
        }

        #[smashline::fighter_frame_callback(main)]
        fn #runtime_name(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
            #[allow(unused_unsafe)]
            unsafe {
                let category = smash::app::utility::get_category(&mut *fighter.module_accessor);
                let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
                if category == *smash::lib::lua_const::BATTLE_OBJECT_CATEGORY_FIGHTER && kind == *#attrs {
                    #usr_fn_name(fighter)
                }
            }
        }

        pub fn install(is_runtime: bool) {
            if is_runtime {
                smashline::install_agent_frame_callback!(#runtime_name);
            } else {
                smashline::install_agent_frame!(#static_name);
            }
        }
    ).into()
}

#[proc_macro]
pub fn size_of_rom_file(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(
                TokenStream2::from(item),
                format!("Failed to parse input as string literal: {:?}", e),
            )
            .into_compile_error()
            .into();
        }
    };
    let local_path = literal.value();

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.")
                .into_compile_error()
                .into()
        }
    };

    let complete_path = parent.join("../../romfs/build/").join(local_path);
    match std::fs::metadata(&complete_path) {
        Ok(metadata) => syn::LitInt::new(format!("{}", metadata.len()).as_str(), literal.span())
            .to_token_stream()
            .into(),
        Err(e) => syn::Error::new(
            literal.span(),
            format!(
                "Could not get file size of path {}: {:?}",
                complete_path.display(),
                e
            ),
        )
        .into_compile_error()
        .into(),
    }
}

#[proc_macro]
pub fn rom_path(item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(item.span(), "Failed to get parent of current path.")
                .into_compile_error()
                .into()
        }
    };

    let full_path = parent.join("../../romfs/build/");

    let full_path = match full_path.as_os_str().to_str() {
        Some(path) => path,
        None => {
            return syn::Error::new(item.span(), "Full path contains invalid characters!")
                .into_compile_error()
                .into()
        }
    };

    syn::LitStr::new(full_path, item.span())
        .to_token_stream()
        .into()
}

#[proc_macro]
pub fn rom_source_path(item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(item.span(), "Failed to get parent of current path.")
                .into_compile_error()
                .into()
        }
    };

    let full_path = parent.join("../../romfs/source/");

    let full_path = match full_path.as_os_str().to_str() {
        Some(path) => path,
        None => {
            return syn::Error::new(item.span(), "Full path contains invalid characters!")
                .into_compile_error()
                .into()
        }
    };

    syn::LitStr::new(full_path, item.span())
        .to_token_stream()
        .into()
}

#[proc_macro]
pub fn from_root(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(
                TokenStream2::from(item),
                format!("Failed to parse input as string literal: {:?}", e),
            )
            .into_compile_error()
            .into();
        }
    };

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.")
                .into_compile_error()
                .into()
        }
    };

    let full_path = parent.join("../../").join(literal.value());

    let full_path = match full_path.as_os_str().to_str() {
        Some(path) => path,
        None => {
            return syn::Error::new(literal.span(), "Full path contains invalid characters!")
                .into_compile_error()
                .into()
        }
    };

    syn::LitStr::new(full_path, literal.span())
        .to_token_stream()
        .into()
}

#[proc_macro]
pub fn hash40(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(
                TokenStream2::from(item),
                format!("Failed to parse input as string literal: {:?}", e),
            )
            .into_compile_error()
            .into();
        }
    };

    let str = literal.value();
    let hash = hash40::hash40(str.as_str());
    syn::LitInt::new(format!("{}", hash.0).as_str(), literal.span())
        .to_token_stream()
        .into()
}

#[proc_macro]
pub fn agent_params(item: TokenStream) -> TokenStream {
    let literal = match syn::parse::<syn::LitStr>(item.clone()) {
        Ok(literal) => literal,
        Err(e) => {
            return syn::Error::new_spanned(
                TokenStream2::from(item),
                format!("Failed to parse input as string literal: {:?}", e),
            )
            .into_compile_error()
            .into();
        }
    };

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.")
                .into_compile_error()
                .into()
        }
    };

    let full_path = parent.join("../../").join(literal.value());

    let data = match std::fs::read_to_string(&full_path) {
        Ok(result) => result,
        Err(e) => {
            return syn::Error::new(
                literal.span(),
                format!("Failed to open file at '{}': {:?}", full_path.display(), e),
            )
            .into_compile_error()
            .into();
        }
    };

    let path = Path::new(file!());
    let parent = match path.parent() {
        Some(parent) => parent,
        None => {
            return syn::Error::new(literal.span(), "Failed to get parent of current path.")
                .into_compile_error()
                .into()
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
                    writeln!(output, "fighter_kind_{}:{}:{}", agent, file, metadata.len())
                        .expect("Unknown error writing to output!");
                } else {
                    panic!("Missing romfs file {}", file);
                }
            }
        }
    }
    syn::LitStr::new(output.as_str(), literal.span())
        .to_token_stream()
        .into()
}

// fn recreate_module_paths(paths: &syn::punctuated::Punctuated<syn::Path, syn::token::Comma>) -> syn::Item {
//     enum PathTerminator {
//         Mod(HashMap<String, PathTerminator>),
//         Item(PathTerminator)
//     }

//     let mut module = None;

//     let mut hashmap = HashMap::new();

//     for path in paths.iter() {
//         for segment in path.segments.iter().rev().skip(1) {

//         }
//     }

//     for path in paths.iter() {
//         // reverse the path so that the last segment is first and then skip it since it's a regular item
//         for segment in path.segments.iter().rev().skip(1) {
//             if let Some(mod_) = module {
//                 module = Some(syn::parse_quote!(
//                     mod #segment { pub #mod_ }
//                 ));
//             } else {
//                 module = Some(syn::parse_quote!( mod #segment { #end } ));
//             }
//         }
//     }

//     if let Some(module) = module {
//         syn::Item::Mod(module)
//     } else {
//         end
//     }
// }

// struct ArgList(syn::punctuated::Punctuated<syn::Path, syn::token::Comma>);

// impl syn::parse::Parse for ArgList {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         syn::punctuated::Punctuated::parse_terminated(input).map(|x| Self(x))
//     }
// }

fn handle_tree_path(path: syn::UsePath, current_path: &String, no_ret: bool) -> syn::Item {
    let syn::UsePath { tree, ident, .. } = path;
    let mut new_mod: syn::ItemMod = syn::parse_quote!(
        pub mod #ident {

        }
    );
    new_mod
        .content
        .as_mut()
        .unwrap()
        .1
        .push(handle_tree_recursive(
            Box::into_inner(tree),
            &format!("{}__{}", current_path, ident.to_string()),
            no_ret,
        ));
    syn::Item::Mod(new_mod)
}

fn handle_tree_group(group: syn::UseGroup, current_path: &String, no_ret: bool) -> syn::Item {
    let syn::UseGroup { items, .. } = group;
    let items: Vec<syn::Item> = items
        .into_iter()
        .map(|x| handle_tree_recursive(x, current_path, no_ret))
        .collect();
    let items2 = items.iter();
    syn::Item::Verbatim(quote::quote!(
        #(
            #items2
        )*
    ))
}

fn handle_tree_rename(rename: syn::UseRename, current_path: &String, no_ret: bool) -> syn::Item {
    let og_fn_name = &rename.ident;
    let new_fn_name = &rename.rename;
    let link_name = syn::LitStr::new(
        format!("{}__{}", current_path, og_fn_name.to_string()).as_str(),
        og_fn_name.span(),
    );
    if no_ret {
        syn::parse_quote!(
            extern "C" {
                #[link_name = #link_name]
                pub fn #new_fn_name(fighter: &mut smash::lua2cpp::L2CFighterCommon);
            }
        )
    } else {
        syn::parse_quote!(
            extern "C" {
                #[link_name = #link_name]
                pub fn #new_fn_name(fighter: &mut smash::lua2cpp::L2CFighterCommon) -> smash::lib::L2CValue;
            }
        )
    }
}

fn handle_tree_name(name: syn::UseName, current_path: &String, no_ret: bool) -> syn::Item {
    let fn_name = &name.ident;
    let link_name = syn::LitStr::new(
        format!("{}__{}", current_path, fn_name.to_string()).as_str(),
        name.span(),
    );
    if no_ret {
        syn::parse_quote!(
            extern "C" {
                #[link_name = #link_name]
                pub fn #fn_name(fighter: &mut smash::lua2cpp::L2CFighterCommon);
            }
        )
    } else {
        syn::parse_quote!(
            extern "C" {
                #[link_name = #link_name]
                pub fn #fn_name(fighter: &mut smash::lua2cpp::L2CFighterCommon) -> smash::lib::L2CValue;
            }
        )
    }
}

fn handle_tree_recursive(tree: syn::UseTree, current_path: &String, no_ret: bool) -> syn::Item {
    match tree {
        syn::UseTree::Glob(glob) => syn::Item::Verbatim(
            syn::Error::new(glob.span(), "HDR Imports cannot use the glob match!")
                .into_compile_error()
                .into_token_stream(),
        ),
        syn::UseTree::Path(path) => handle_tree_path(path, current_path, no_ret),
        syn::UseTree::Group(group) => handle_tree_group(group, current_path, no_ret),
        syn::UseTree::Rename(rename) => handle_tree_rename(rename, current_path, no_ret),
        syn::UseTree::Name(name) => handle_tree_name(name, current_path, no_ret),
    }
}

#[proc_macro]
pub fn import_noreturn(item: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(item as syn::UseTree);
    handle_tree_recursive(tree, &String::from("hdr"), true)
        .to_token_stream()
        .into()
}

#[proc_macro]
pub fn import(item: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(item as syn::UseTree);
    handle_tree_recursive(tree, &String::from("hdr"), false)
        .to_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as syn::Path);
    let mut usr_fn = parse_macro_input!(item as syn::ItemFn);
    let mut link_name = String::new();
    for seg in path.segments.iter() {
        link_name += &format!("__{}", seg.ident.to_string());
    }
    if !link_name.starts_with("hdr") {
        link_name = "hdr".to_string() + &link_name + &format!("__{}", usr_fn.sig.ident.to_string());
    }
    let link_name = syn::LitStr::new(link_name.as_str(), path.span());
    let public = syn::VisPublic {
        pub_token: syn::token::Pub { span: path.span() },
    };
    usr_fn.vis = syn::Visibility::Public(public);
    usr_fn.sig.abi = Some(syn::Abi {
        extern_token: syn::token::Extern { span: path.span() },
        name: Some(syn::LitStr::new("C", path.span())),
    });
    usr_fn
        .attrs
        .push(syn::parse_quote!(#[export_name = #link_name]));

    usr_fn.to_token_stream().into()
}
