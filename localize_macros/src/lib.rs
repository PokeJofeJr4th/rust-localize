use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, LitStr, Result, Token,
};

struct TranslationInput {
    struct_name: Ident,
    strings: HashMap<String, HashMap<Ident, LitStr>>,
    locales: HashSet<Ident>,
}

struct TranslationItem {
    key: LitStr,
    values: Punctuated<TranslationValue, Token![,]>,
}

struct TranslationValue {
    locale: Ident,
    value: LitStr,
}

impl Parse for TranslationInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let syntax_type: Ident = input.parse()?;
        match &*syntax_type.to_string() {
            "LDSL" => {
                let translations = input.parse_terminated(TranslationItem::parse, Token![,])?;
                let mut strings: HashMap<String, HashMap<Ident, LitStr>> = HashMap::new();
                let mut locales: HashSet<Ident> = HashSet::new();
                for item in translations {
                    let key = item.key.value();
                    let mut current_string = HashMap::new();
                    for translation in item.values {
                        if current_string
                            .insert(translation.locale.unraw(), translation.value)
                            .is_some()
                        {
                            return Err(syn::Error::new(
                                translation.locale.span(),
                                "Duplicate locale identifier in translation",
                            ));
                        }
                        locales.insert(translation.locale.unraw());
                    }
                    strings.insert(key, current_string);
                }

                Ok(TranslationInput {
                    struct_name,
                    strings,
                    locales,
                })
            }
            _ => todo!(),
        }
    }
}

impl Parse for TranslationItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: LitStr = input.parse()?;
        let content;
        syn::braced!(content in input);
        let values = content.parse_terminated(TranslationValue::parse, Token![,])?;
        Ok(TranslationItem { key, values })
    }
}

impl Parse for TranslationValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let locale: Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let value: LitStr = input.parse()?;
        Ok(TranslationValue { locale, value })
    }
}

#[proc_macro]
pub fn localization_table(table: TokenStream) -> TokenStream {
    let TranslationInput {
        struct_name,
        strings,
        locales,
    } = parse_macro_input!(table as TranslationInput);
    let locales: Vec<_> = locales.into_iter().collect();
    let num_keys = strings.len();
    let num_locales = locales.len();
    let translation_keys: Vec<String> = strings.keys().cloned().collect();
    let translations: Vec<_> = locales
        .iter()
        .map(|loc| {
            let translations: Vec<String> = translation_keys
                .iter()
                .map(|key| {
                    strings
                        .get(key)
                        .and_then(|x| x.get(loc).map(LitStr::value))
                        .unwrap_or_else(|| String::from("<NO TRANSLATION>"))
                })
                .collect();
            quote! {[#(#translations),*]}
        })
        .collect();
    let locales: Vec<String> = locales
        .into_iter()
        .map(|id: Ident| id.to_string())
        .collect();
    quote! {
        pub struct #struct_name;

        impl #struct_name {
            pub const TABLE: ::localize::LocalizationTable<'static, #num_locales, #num_keys> = ::localize::LocalizationTable {
                translation_keys: [#(#translation_keys),*],
                locales: [#(#locales),*],
                translations: [#(#translations),*],
            };

            pub const fn localize(translation_key: &str, locale: &str) -> &'static str {
                Self::TABLE.localize(translation_key, locale)
            }

            pub const fn get_locale(locale: &str) -> ::localize::LocaleHandle<'static, #num_keys> {
                Self::TABLE.get_locale(locale)
            }
        }
    }.into()
}
