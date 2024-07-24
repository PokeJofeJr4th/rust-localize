//! # Localize
//! A crate for creating localization tables for static strings. The core data model for this crate is as follows.
//! See other sections of the documentation for more information about how to use specific features.
//!
//! * A **translation key** is a string literal that uniquely identifies a translation string.
//!   * The special translation key `"_"` creates a default translation to be used when a translation isn't specified.
//! * A **locale** is an identifier, often two letters long, that uniquely identifies a set of strings that the
//!   table should be able to switch between.
//! * A **translation** is a user-facing string literal corresponding to a given translation key and locale.
//!
//! # Example
//! This example shows one very simple use of `localize`. For more examples, see the relevant macro, struct,
//! and function documentation.
//! ```
//! use localize::{localization_table, LocalizationTable};
//!
//! localization_table!{Spanglish = LDSL {
//!     "greeting" = {
//!         en => "Hello",
//!         es => "Hola"
//!     },
//!     "farewell" = {
//!         en => "Goodbye",
//!         es => "Adiós"
//!     }
//! }}
//!
//! let greeting_en = Spanglish::localize("greeting", "en");
//! assert_eq!(greeting_en, "Hello");
//!
//! let greeting_es = Spanglish::localize("greeting", "es");
//! assert_eq!(greeting_es, "Hola");
//!
//! let farewell_en = Spanglish::localize("farewell", "en");
//! assert_eq!(farewell_en, "Goodbye");
//!
//! let farewell_es = Spanglish::localize("farewell", "es");
//! assert_eq!(farewell_es, "Adiós");
//! ```

#![warn(clippy::pedantic, clippy::nursery)]
pub use localize_macros::localization_table;
use std::fmt::Display;

/// A table of translations based on locale.
///
/// The best way to generate this struct is through the `localization_table` macro,
/// which provides a simple syntax and guarantees that the translation keys and locales are formatted properly.
/// # Example
///
/// ```
/// # use localize::{localization_table, LocalizationTable};
///
/// # localization_table!{Spanglish = LDSL {
/// #    "greeting" = {
/// #        en => "Hello",
/// #        es => "Hola"
/// #    },
/// #    "farewell" = {
/// #        en => "Goodbye",
/// #        es => "Adiós"
/// #    }
/// # }}
/// let spanglish: LocalizationTable<'static, 2, 2>;
/// // ...
/// # spanglish = Spanglish::TABLE;
///
/// let greeting_en = spanglish.localize("greeting", "en");
/// assert_eq!(greeting_en, "Hello");
///
/// let greeting_es = spanglish.localize("greeting", "es");
/// assert_eq!(greeting_es, "Hola");
///
/// let farewell_en = spanglish.localize("farewell", "en");
/// assert_eq!(farewell_en, "Goodbye");
///
/// let farewell_es = spanglish.localize("farewell", "es");
/// assert_eq!(farewell_es, "Adiós");
/// ```
#[derive(Clone, Copy)]
pub struct LocalizationTable<'a, const LOCALES: usize, const KEYS: usize> {
    pub translation_keys: [&'a str; KEYS],
    pub locales: [&'a str; LOCALES],
    pub translations: [[&'a str; KEYS]; LOCALES],
}

impl<'a, const LOCALES: usize, const KEYS: usize> LocalizationTable<'a, LOCALES, KEYS> {
    #[inline]
    #[must_use]
    /// Translates a given key to the corresponding localized string for the specified locale.
    ///
    /// # Parameters
    ///
    /// - `key`: The translation key, which identifies the string to be translated.
    /// - `locale`: The locale for which the translation is requested.
    ///
    /// # Returns
    ///
    /// - A reference to the localized string corresponding to the key and locale.
    /// - If the translation is not available for the specified locale and the default locale,
    ///   an empty string is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use localize::localization_table;
    ///
    /// # localization_table!{Spanglish = LDSL {
    /// #    "greeting" = {
    /// #        en => "Hello",
    /// #        es => "Hola"
    /// #    },
    /// #    "farewell" = {
    /// #        en => "Goodbye",
    /// #        es => "Adiós"
    /// #    }
    /// # }}
    /// # let spanglish = Spanglish::TABLE;
    ///
    /// let greeting_en = spanglish.localize("greeting", "en");
    /// assert_eq!(greeting_en, "Hello");
    ///
    /// let greeting_es = spanglish.localize("greeting", "es");
    /// assert_eq!(greeting_es, "Hola");
    ///
    /// let farewell_en = spanglish.localize("farewell", "en");
    /// assert_eq!(farewell_en, "Goodbye");
    ///
    /// let farewell_es = spanglish.localize("farewell", "es");
    /// assert_eq!(farewell_es, "Adiós");
    /// ```
    pub const fn localize(&self, translation_key: &str, locale: &str) -> &'a str {
        self.translations[find_idx(&self.locales, locale)]
            [find_idx(&self.translation_keys, translation_key)]
    }

    /// Create a reference to the specified locale
    /// # Example
    /// ```
    /// # use localize::{localization_table, LocaleHandle};
    ///
    /// localization_table!{Spanglish = LDSL {
    ///    "greeting" = {
    ///        en => "Hello",
    ///        es => "Hola"
    ///    },
    ///    "farewell" = {
    ///        en => "Goodbye",
    ///        es => "Adiós"
    ///    }
    /// }}
    ///
    /// let spanish = Spanglish::get_locale("es");
    /// assert_eq!(spanish.localize("greeting"), "Hola");
    ///
    /// let english = Spanglish::get_locale("en");
    /// assert_eq!(english.localize("greeting"), "Hello");
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_locale(&'a self, locale: &str) -> LocaleHandle<'a, KEYS> {
        let idx = find_idx(&self.locales, locale);
        LocaleHandle {
            locale: self.locales[idx],
            translation_keys: &self.translation_keys,
            translations: &self.translations[idx],
        }
    }
}

/// A reference to a specific row of a translation table.
///
/// # Example
/// ```
/// # use localize::{localization_table, LocaleHandle};
///
/// localization_table!{Spanglish = LDSL {
///    "greeting" = {
///        en => "Hello",
///        es => "Hola"
///    },
///    "farewell" = {
///        en => "Goodbye",
///        es => "Adiós"
///    }
/// }}
///
/// let spanish: LocaleHandle<'static, 2> = Spanglish::get_locale("es");
/// assert_eq!(spanish.localize("greeting"), "Hola");
/// assert_eq!(format!("{spanish}"), "es");
///
/// let english: LocaleHandle<'static, 2> = Spanglish::get_locale("en");
/// assert_eq!(english.localize("greeting"), "Hello");
/// assert_eq!(format!("{english}"), "en");
/// ```
#[derive(Clone, Copy)]
pub struct LocaleHandle<'a, const KEYS: usize> {
    locale: &'a str,
    translation_keys: &'a [&'a str; KEYS],
    translations: &'a [&'a str; KEYS],
}

impl<'a, const KEYS: usize> Display for LocaleHandle<'a, KEYS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.locale)
    }
}

impl<'a, const KEYS: usize> LocaleHandle<'a, KEYS> {
    /// Get the translated string for the given translation key in this locale
    #[inline]
    #[must_use]
    pub const fn localize(&self, translation_key: &str) -> &'a str {
        self.translations[find_idx(self.translation_keys, translation_key)]
    }
}

#[inline]
const fn strcmp(a: &str, b: &str) -> bool {
    a.len() == b.len() && {
        let mut i = 0;
        while i < a.len() {
            if a.as_bytes()[i] != b.as_bytes()[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

const fn find_idx(arr: &[&str], s: &str) -> usize {
    let mut i = 0;
    while i < arr.len() {
        if strcmp(arr[i], s) {
            return i;
        }
        i += 1;
    }
    0
}
