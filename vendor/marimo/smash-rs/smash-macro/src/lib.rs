use proc_macro::{TokenStream};
use syn::{parse_macro_input, token::*, spanned::Spanned, parse::Parse};
use quote::ToTokens;

#[proc_macro_attribute]
pub fn repr_weak(attr: TokenStream, item: TokenStream) -> TokenStream {
    let repr = parse_macro_input!(attr as syn::Type);
    let en = parse_macro_input!(item as syn::ItemEnum);

    let fields = en.variants.iter().filter_map(|x| {
        x.discriminant
            .as_ref()
            .map(|(_, expr)| (&x.ident, expr))
    });

    let statements = fields.map(|(id, expr)| {
        let statement: syn::ItemConst = syn::parse_quote!(pub const #id: #repr = #expr;);
        statement
    });

    let vis = &en.vis;

    let enum_name = &en.ident;

    let mut module: syn::ItemMod = syn::parse_quote!(
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #vis mod #enum_name {

        }
    );

    for stmt in statements {
        if let Some((_, items)) = module.content.as_mut() {
            items.push(syn::Item::Const(stmt));
        }
    }

    module.into_token_stream().into()
}

fn bare_fn_to_fn(attrs: &Vec<syn::Attribute>, name: &syn::Ident, public: &syn::Visibility, bare: &syn::TypeBareFn) -> Result<syn::ItemFn, syn::Error> {
    // basic idea:
    // 1. Get the argument named "this". This argument is going to become the self argument, and we base
    //      the mutability of the function on whether or not this is &Type or &mut Type

    let mut self_arg = syn::FnArg::Receiver(syn::Receiver {
        attrs: vec![],
        reference: Some((And(bare.fn_token.span()), None)),
        mutability: Some(Mut(bare.fn_token.span())),
        self_token: SelfValue(bare.fn_token.span())
    });

    let mut found_this = false;

    let mut remaining_args: syn::punctuated::Punctuated<&syn::BareFnArg, syn::token::Comma> = syn::punctuated::Punctuated::new();
    for arg in bare.inputs.iter() {
        let arg_name = match &arg.name {
            Some(name) => name,
            None => return Err(syn::Error::new(bare.span(), "This function must have named arguments"))
        };

        if arg_name.0.to_string() == "this" {
            let ty = &arg.ty;
            match ty {
                syn::Type::Reference(ref_type) => {
                    if let syn::FnArg::Receiver(self_arg) = &mut self_arg {
                        self_arg.mutability = ref_type.mutability.clone();
                    } else {
                        unreachable!()
                    }
                },
                _ => {
                    return Err(syn::Error::new(bare.span(), "This function must take a reference type named 'this'!"));
                }
            }
            found_this = true;
        } else {
            remaining_args.push(arg);
        }
    }

    if !found_this {
        return Err(syn::Error::new(bare.span(), "This function must take a reference type named 'this'!"));
    }

    let ret = &bare.output;

    let mut arg_names: syn::punctuated::Punctuated<&syn::Ident, syn::token::Comma> = syn::punctuated::Punctuated::new();
    for arg in remaining_args.iter() {
        arg_names.push(&arg.name.as_ref().unwrap().0);
    }

    let attrs = attrs.iter().filter_map(|attr| {
        if TypeAssertAttributes::parse_attr(attr, "size").is_ok()
            || TypeAssertAttributes::parse_attr(attr, "off").is_ok()
            || TypeAssertAttributes::parse_attr(attr, "offset").is_ok()
        {
            None
        } else {
            Some(attr)
        }
    });

    Ok(syn::parse_quote!{
        #(#attrs)*
        #public fn #name(#self_arg, #remaining_args) #ret {
            unsafe {
                (self.vtable. #name)(self, #arg_names)
            }
        }
    })
}

#[proc_macro_attribute]
pub fn vtable_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ty_name = parse_macro_input!(attr as syn::Ident);
    let item = parse_macro_input!(item as syn::ItemStruct);

    let funcs: Vec<(&Vec<syn::Attribute>, &syn::Ident, &syn::Visibility, &syn::TypeBareFn)> = item.fields.iter().filter_map(|field| {
        if field.ident.is_none() {
            return None;
        }
        
        match &field.ty {
            syn::Type::BareFn(bare_fn) => Some((&field.attrs, field.ident.as_ref().unwrap(), &field.vis, bare_fn)),
            _ => None
        }
    }).collect();

    let mut new_funcs = vec![];
    for (attrs, ident, vis, bare_fn) in funcs.into_iter() {
        match bare_fn_to_fn(attrs, ident, vis, bare_fn) {
            Ok(func) => new_funcs.push(func),
            Err(e) => return e.into_compile_error().into_token_stream().into()
        }
    }

    let funcs = new_funcs.iter();

    quote::quote!(
        #item

        impl #ty_name {
            #(#funcs)*
        }
    ).into()
}

#[proc_macro_attribute]
pub fn virtual_implementor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ty_name = parse_macro_input!(attr as syn::Ident);
    let item = parse_macro_input!(item as syn::ItemStruct);

    let item_name = &item.ident;

    quote::quote!(
        #item

        impl std::ops::Deref for #item_name {
            type Target = #ty_name;

            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        } 

        impl std::ops::DerefMut for #item_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.parent
            }
        }
    ).into()
}

struct TypeAssertAttributes(syn::LitInt);

impl TypeAssertAttributes {
    fn parse_attr(attr: &syn::Attribute, name: &str) -> syn::Result<Self> {
        match attr.path.segments.last() {
            Some(seg) if seg.ident.to_string() == name => {},
            _ => return Err(syn::Error::new(attr.path.span(), "Type assert attribute requires 'ta' ident"))
        }

        syn::parse(attr.tokens.to_token_stream().into())
    }
}

impl Parse for TypeAssertAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![=] = input.parse()?;

        input.parse().map(|int| Self(int))
    }
}

#[proc_macro_derive(TypeAssert, attributes(size, off, offset))]
pub fn derive_type_assertion(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as syn::ItemStruct);

    let mut size = None;

    for attr in item_struct.attrs.iter() {
        if let Ok(attr) = TypeAssertAttributes::parse_attr(attr, "size") {
            size = Some(attr.0);
            break;
        }
    }

    let size = match size {
        Some(int) => int,
        None => return syn::Error::new(item_struct.ident.span(), "TypeAssert structure must have 'ta' attribute to specify the size").into_compile_error().into()
    };

    let fields: Vec<(&syn::Ident, syn::LitInt)> = item_struct.fields.iter().filter_map(|field| {
        if field.ident.is_none() {
            return None;
        }

        let mut attributes = field.attrs.iter();

        loop {
            match attributes.next() {
                Some(attr) => {
                    match TypeAssertAttributes::parse_attr(attr, "off") {
                        Ok(attr) => break Some((field.ident.as_ref().unwrap(), attr.0)),
                        _ => match TypeAssertAttributes::parse_attr(attr, "offset") {
                            Ok(attr) => break Some((field.ident.as_ref().unwrap(), attr.0)),
                            _ => {}
                        }
                    }
                },
                None => break None
            }
        }
    }).collect();
    let ty_ident = &item_struct.ident;

    let exprs = fields
        .into_iter()
        .map(|(ident, val)| {
            syn::parse_quote!{ 
                assert_eq!(
                    offset_of!(#ty_ident, #ident),
                    #val,
                    "The offset of {}.{} must be {:#x}, but it is {:#x}",
                    stringify!(#ty_ident),
                    stringify!(#ident),
                    #val,
                    offset_of!(#ty_ident, #ident)
                );
            }
        })
        .collect::<Vec<syn::Stmt>>()
        .into_iter();

    let test_mod_name = quote::format_ident!("{}_tests", ty_ident);

    quote::quote!(
        #[cfg(feature = "type_assert")]
        impl #ty_ident {
            pub fn assert(is_size_assert: bool) {
                if is_size_assert {
                    assert_eq!(size_of!(#ty_ident), #size, "The size of {} must be {:#x}, but it is {:#x}", stringify!(#ty_ident), #size, size_of!(#ty_ident));
                    return;
                }
                #(
                    #exprs
                )*
            }
        }

        #[cfg(feature = "type_assert")]
        #[allow(non_snake_case)]
        mod #test_mod_name {
            use super::*;

            #[test]
            pub fn check_size_field_bounds() {
                #ty_ident::assert(false);
                #ty_ident::assert(true);
            }
        }
    ).into()
}