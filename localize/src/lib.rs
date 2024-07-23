#![warn(clippy::pedantic, clippy::nursery)]
pub use localize_macros::localization_table;

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
    /// # Examples
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

    #[inline]
    #[must_use]
    pub const fn get_locale(&'a self, locale: &str) -> LocaleHandle<'a, KEYS> {
        LocaleHandle {
            translation_keys: &self.translation_keys,
            translations: &self.translations[find_idx(&self.translation_keys, locale)],
        }
    }
}

#[derive(Clone, Copy)]
pub struct LocaleHandle<'a, const KEYS: usize> {
    translation_keys: &'a [&'a str; KEYS],
    translations: &'a [&'a str; KEYS],
}

impl<'a, const KEYS: usize> LocaleHandle<'a, KEYS> {
    #[inline]
    #[must_use]
    pub const fn localize(&self, translation_key: &str) -> &'a str {
        self.translations[find_idx(self.translation_keys, translation_key)]
    }
}

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
