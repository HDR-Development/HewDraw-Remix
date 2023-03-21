use crate::meta::MetaItem;
use proc_macro2::Span;
use syn::{parse::Parse, spanned::Spanned};

mod kw {
    syn::custom_keyword!(object);
}

pub enum WorkAttribute {
    Object(syn::LitStr),
}

macro_rules! parse_attrs {
    (kv: $k:path, $v:path, $var:ident, $inner:ident) => {
        if $inner.peek($k) {
            return MetaItem::<$k, $v>::parse($inner).map(|m| Self::$var(m.into_value()));
        }
    };
    (flag: $t:path, $var:ident, $inner:ident) => {
        if $inner.peek($t) {
            let _: $t = $inner.parse()?;
            return Ok(Self::$var);
        }
    };
    ($(($($t:tt)*)),*) => {
        impl Parse for WorkAttribute {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                $(
                    parse_attrs!($($t)*, input);
                )*

                Err(syn::Error::new(
                    input.span(),
                    "Invalid #[work(...)] attribute"
                ))
            }
        }
    }
}

parse_attrs! {
    (kv: kw::object, syn::LitStr, Object)
}

impl WorkAttribute {
    pub fn collect(attrs: &[syn::Attribute]) -> Vec<(Span, syn::Result<Self>)> {
        attrs
            .iter()
            .filter(|attr| {
                attr.path()
                    .get_ident()
                    .map(|ident| ident == "work")
                    .unwrap_or(false)
            })
            .map(|attr| (attr.span(), attr.parse_args::<Self>()))
            .collect()
    }
}
