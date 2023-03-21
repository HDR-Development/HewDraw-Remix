use attr::WorkAttribute;
use proc_macro::TokenStream as TS;
use proc_macro2::{Span, TokenStream};
use proc_macro_crate::FoundCrate;
use proc_macro_error::{abort, emit_error, proc_macro_error};
use quote::quote;
use syn::Type;

mod attr;
mod meta;

fn find_params_crate() -> TokenStream {
    match proc_macro_crate::crate_name("params") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(name.as_str(), Span::call_site());
            quote!(::#ident)
        }
        Err(error) => abort!(Span::call_site(), error),
    }
}

fn call_for_type(ty: &Type, ident: &syn::Ident, params_crate: &TokenStream) -> TokenStream {
    let int = quote! {
        unsafe {
            #params_crate::__private::smash::app::lua_bind::WorkModule::get_param_int(
                module_accessor,
                #params_crate::__private::smash::hash40(object_name),
                #params_crate::__private::smash::hash40(stringify!(#ident))
            )
        }
    };

    let float = quote! {
        unsafe {
            #params_crate::__private::smash::app::lua_bind::WorkModule::get_param_float(
                module_accessor,
                #params_crate::__private::smash::hash40(object_name),
                #params_crate::__private::smash::hash40(stringify!(#ident))
            )
        }
    };

    let long = quote! {
        unsafe {
            #params_crate::__private::smash::app::lua_bind::WorkModule::get_param_int64(
                module_accessor,
                #params_crate::__private::smash::hash40(object_name),
                #params_crate::__private::smash::hash40(stringify!(#ident))
            )
        }
    };

    match ty {
        Type::Path(syn::TypePath { path, .. }) => {
            match path.segments.last().map(|seg| &seg.ident) {
                Some(ident) => {
                    if ident == "bool" {
                        quote!(#int != 0)
                    } else if ["i8", "u8", "i16", "u16", "u32"]
                        .into_iter()
                        .any(|ty| ident == ty)
                    {
                        quote!(#int.try_into().unwrap())
                    } else if ident == "i32" {
                        quote!(#int)
                    } else if ident == "u64" {
                        quote!(#long.try_into().unwrap())
                    } else if ident == "i64" {
                        quote!(#long)
                    } else if ident == "Hash40" {
                        quote!(#params_crate::__private::smash::phx::Hash40::new_raw(#long as u64))
                    } else if ident == "f32" {
                        quote!(#float)
                    } else if ident == "f64" {
                        quote!(#float.try_into().unwrap())
                    } else {
                        emit_error!(ty, "invalid type");
                        quote!(unimplemented!())
                    }
                }
                None => {
                    emit_error!(ty, "invalid_type");
                    quote!(unimplemented!())
                }
            }
        }
        _ => {
            emit_error!(ty, "invalid_type");
            quote!(unimplemented!())
        }
    }
}

#[proc_macro_error]
#[proc_macro_derive(WorkParams, attributes(work))]
pub fn derive_work_params(item: TS) -> TS {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let params_crate = find_params_crate();

    let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }), .. }) = &input.data else {
        abort!(input, "#[derive(WorkParams)] must only be used on a structure with named fields");
    };

    let ident = &input.ident;

    let mut object_name = None;
    for (span, attr) in WorkAttribute::collect(&input.attrs) {
        let attr = match attr {
            Ok(a) => a,
            Err(e) => {
                emit_error!(span, e);
                continue;
            }
        };

        match attr {
            WorkAttribute::Object(object) => {
                if object_name.replace(object).is_some() {
                    emit_error!(span, "only use #[work(object = ...)] once per struct");
                }
            }
        }
    }

    let field_idents = fields.iter().map(|field| field.ident.as_ref().unwrap());

    let parse_stmts = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let parse = call_for_type(&field.ty, ident, &params_crate);

        quote! {
            let #ident = #parse;
        }
    });

    let param_object = if let Some(name) = object_name {
        quote! {
            impl #params_crate::WorkParamObject for #ident {
                const NAME: &'static str = #name;
            }
        }
    } else {
        quote!()
    };

    TS::from(quote! {
        #param_object

        impl #params_crate::WorkParams for #ident {
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn parse(module_accessor: *mut #params_crate::__private::smash::app::BattleObjectModuleAccessor, object_name: &'static str) -> Self {
                #(#parse_stmts)*

                Self {
                    #(#field_idents,)*
                }
            }
        }
    })
}
