use proc_macro::TokenStream as TS;
use proc_macro2::{Span, TokenStream};
use proc_macro_crate::FoundCrate;
use proc_macro_error::{abort, emit_error, proc_macro_error};
use quote::quote;
use syn::{parse::Parse, spanned::Spanned, Attribute};

fn find_variables_crate() -> TokenStream {
    match proc_macro_crate::crate_name("variables") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(name.as_str(), Span::call_site());
            quote!(::#ident)
        }
        Err(error) => abort!(Span::call_site(), error),
    }
}

mod kw {
    syn::custom_keyword!(extend);
    syn::custom_keyword!(kind);
    syn::custom_keyword!(default);
}

struct Extends(Option<syn::Path>);

impl Parse for Extends {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<kw::extend>().is_err() {
            return Ok(Self(None));
        }
        let _: syn::Token![=] = input.parse()?;
        input.parse().map(|v| Self(Some(v)))
    }
}

struct Status {
    kind: Option<syn::Expr>,
    default: Option<syn::Expr>,
}

impl Status {
    pub fn remove_status(attrs: &mut Vec<Attribute>) -> Option<Self> {
        let mut status = None;
        *attrs = std::mem::take(attrs)
            .into_iter()
            .filter_map(|attr| {
                let Some(id) = attr.path().get_ident() else {
                return Some(attr);
            };

                if id != "status" {
                    return Some(attr);
                }

                let current = match attr.parse_args::<Self>() {
                    Ok(args) => args,
                    Err(e) => {
                        emit_error!(attr.span(), e);
                        return None;
                    }
                };

                if status.replace(current).is_some() {
                    emit_error!(attr.span(), "only specify one #[status(...)] per variant");
                }

                None
            })
            .collect();

        status
    }
}

impl Parse for Status {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kind = if input.parse::<kw::kind>().is_ok() {
            let _: syn::Token![=] = input.parse()?;

            let kind = input.parse()?;

            if input.parse::<syn::Token![,]>().is_err() {
                return Ok(Self {
                    kind: Some(kind),
                    default: None,
                });
            }

            Some(kind)
        } else {
            None
        };

        let _: kw::default = input.parse()?;
        let _: syn::Token![=] = input.parse()?;

        let default = input.parse()?;

        Ok(Self {
            kind,
            default: Some(default),
        })
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn agent_variables(attr: TS, item: TS) -> TS {
    let mut input = syn::parse_macro_input!(item as syn::ItemEnum);

    let crate_name = find_variables_crate();

    let extends = match syn::parse::<Extends>(attr) {
        Ok(extends) => extends.0,
        Err(e) => {
            emit_error!(Span::call_site(), e);
            None
        }
    };

    let mut write_arms = vec![];
    let mut read_arms = vec![];
    let mut indices = vec![];
    let mut status_vars = vec![];
    let mut max_index = if let Some(extends) = extends.as_ref() {
        quote!(#extends::MAX_INDEX)
    } else {
        quote!(#crate_name::VariableIndex { flag: 0, word: 0, dword: 0, float: 0 })
    };

    for variant in input.variants.iter_mut() {
        if variant.discriminant.take().is_some() {
            emit_error!(
                variant.span(),
                "variable enums do not support explicit values"
            );
        }

        let ident = &variant.ident;
        let index_name = quote::format_ident!("{}_index", ident);

        let status = Status::remove_status(&mut variant.attrs);

        if let Some(status) = status.as_ref() {
            let Status { kind, default } = status;
            let default = default
                .clone()
                .unwrap_or_else(|| syn::parse_quote!(::core::default::Default::default()));
            status_vars.push(quote! {
                if status == #kind {
                    (#default).write(#index_name, accessor);
                }
            });
        }

        match &variant.fields {
            syn::Fields::Unit => {
                write_arms.push(quote!(Self::#ident => unimplemented!()));
                read_arms.push(quote!(Self::#ident => unimplemented!()));
                emit_error!(
                    variant.span(),
                    "variable enum variants must have a type associated"
                );
            }
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    emit_error!(
                        variant.span(),
                        "tuple enum variants must have exactly 1 field"
                    );
                    write_arms.push(quote!(Self::#ident => unimplemented!()));
                    read_arms.push(quote!(Self::#ident => unimplemented!()));
                } else {
                    let ty = &fields.unnamed.first().unwrap().ty;
                    let status_check = if let Some(Status { kind, .. }) = status.as_ref() {
                        quote!(if #kind != status { panic!(concat!("cannot access variable ", stringify!(#ident), " outside of status ", #kind))})
                    } else {
                        quote!()
                    };
                    write_arms.push(quote!(Self::#ident => {
                        if ::core::any::TypeId::of::<V>() != ::core::any::TypeId::of::<#ty>() {
                            panic!(concat!("expected type ", stringify!(#ty), " when writing variable ID ", stringify!(#ident)));
                        }
                        #status_check
                        variable.write(Self::#index_name, accessor);
                    }));
                    read_arms.push(quote!(Self::#ident => {
                        if ::core::any::TypeId::of::<V>() != ::core::any::TypeId::of::<#ty>() {
                            panic!(concat!("expected type ", stringify!(#ty), " when reading variable ID ", stringify!(#ident)));
                        }
                        #status_check
                        V::read(Self::#index_name, accessor)
                    }));

                    indices.push(quote!(
                        #[allow(non_upper_case_globals)]
                        const #index_name: #crate_name::VariableIndex = #max_index;
                    ));
                    max_index = quote!(#crate_name::__private::const_add_index(#max_index, <#ty as #crate_name::Variable>::LENGTH));
                }
            }
            syn::Fields::Named(_) => {
                write_arms.push(quote!(Self::#ident => unimplemented!()));
                read_arms.push(quote!(Self::#ident => unimplemented!()));
                emit_error!(
                    variant.span(),
                    "variable enum variants must be a tuple type"
                );
            }
        }

        variant.fields = syn::Fields::Unit;
    }

    let write_arms = write_arms.into_iter();
    let read_arms = read_arms.into_iter();
    let indices = indices.into_iter();

    let input_name = &input.ident;

    TS::from(quote! {
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
        #input

        impl #crate_name::VariableId for #input_name {
            const MAX_INDEX: #crate_name::VariableIndex = #max_index;

            fn write<V: #crate_name::Variable>(self, variable: V, status: i32, accessor: &mut dyn #crate_name::VariableAccessor) {
                match self {
                    #(#write_arms,)*
                }
            }

            fn read<V: #crate_name::Variable>(self, status: i32, accessor: &dyn #crate_name::VariableAccessor) -> V {
                match self {
                    #(#read_arms,)*
                }
            }

            fn clear_status(status: i32, accessor: &mut dyn #crate_name::VariableAccessor) {
                #(#status_vars)*
            }
        }

        impl #input_name {
            #(#indices)*
        }
    })
}
