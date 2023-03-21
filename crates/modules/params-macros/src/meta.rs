// impl taken from https://github.com/blu-dev/derive-from/blob/master/src/meta.rs
use std::marker::PhantomData;

use syn::parse::Parse;

pub struct MetaItem<K: Parse, V: Parse>(V, PhantomData<K>);

impl<K: Parse, V: Parse> Parse for MetaItem<K, V> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: K = input.parse()?;

        let value: V =
            if input.parse::<syn::Token![=]>().is_ok() || input.parse::<syn::Token![:]>().is_ok() {
                input.parse()?
            } else {
                let content;
                syn::parenthesized!(content in input);
                content.parse()?
            };

        Ok(Self(value, PhantomData))
    }
}

impl<K: Parse, V: Parse> MetaItem<K, V> {
    pub fn into_value(self) -> V {
        self.0
    }
}
