#![warn(clippy::pedantic, clippy::nursery)]

use proc_macro::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, LitStr, Result, Token,
};

struct TranslationInput {
    struct_name: Ident,
    strings: HashMap<String, HashMap<Ident, LitStr>>,
    locales: HashSet<Ident>,
}

struct LDSLTranslationItem {
    key: LitStr,
    values: Punctuated<LDSLTranslationValue, Token![,]>,
}

struct LDSLTranslationValue {
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
                let body;
                syn::braced!(body in input);
                let translations = body.parse_terminated(LDSLTranslationItem::parse, Token![,])?;
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

                Ok(Self {
                    struct_name,
                    strings,
                    locales,
                })
            }
            _ => todo!(),
        }
    }
}

impl Parse for LDSLTranslationItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: LitStr = input.parse()?;
        let _: Token![=] = input.parse()?;
        let content;
        syn::braced!(content in input);
        let values = content.parse_terminated(LDSLTranslationValue::parse, Token![,])?;
        Ok(Self { key, values })
    }
}

impl Parse for LDSLTranslationValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let locale: Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let value: LitStr = input.parse()?;
        Ok(Self { locale, value })
    }
}

#[proc_macro]
/// Generates a `LocalizationTabe` struct from a custom set of translations.
///
/// # Syntax
///
/// The macro invocation always starts with an identifier for the translation table, an equals sign,
/// and an identifier for the translation syntax to use. Currently, the only supported syntax is LDSL, described below.
///
/// ## LDSL (Localization Domain-Specific Language)
///
/// ```
/// # use localize_macros::localization_table;
///
/// localization_table! {MyLocalizationTable = LDSL {
///     "key1" = {
///         locale1 => "translation1",
///         locale2 => "translation2",
///     },
///     "key2" = {
///         locale1 => "translation3",
///         locale2 => "translation4",
///     }
/// }}
/// ```
///
/// The DSL allows you to specify translation keys and their corresponding translations
/// for different locales in a structured and readable format.
///
/// - Each translation key is a string literal.
/// - Each locale is an identifier followed by `=>` and a string literal representing the translation.
///
/// # Example
///
/// ```
/// # use localize_macros::localization_table;
///
/// localization_table! {Spanglish = LDSL {
///     "greeting" = {
///         en => "Hello",
///         es => "Hola",
///     },
///     "farewell" = {
///         en => "Goodbye",
///         es => "Adiós",
///     }
/// }}
///
/// let greeting_en = Spanglish::localize("greeting", "en");
/// assert_eq!(greeting_en, "Hello");
///
/// let greeting_es = Spanglish::localize("greeting", "es");
/// assert_eq!(greeting_es, "Hola");
///
/// let farewell_en = Spanglish::localize("farewell", "en");
/// assert_eq!(farewell_en, "Goodbye");
///
/// let farewell_es = Spanglish::localize("farewell", "es");
/// assert_eq!(farewell_es, "Adiós");
/// ```
///
/// # Errors
///
/// - If the macro input is not well-formed, a compile-time error will be generated.
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
