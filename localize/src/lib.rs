#[derive(Clone, Copy)]
pub struct LocalizationTable<'a, const LOCALES: usize, const KEYS: usize> {
    pub translation_keys: [&'a str; KEYS],
    pub locales: [&'a str; LOCALES],
    pub translations: [[&'a str; KEYS]; LOCALES],
}

impl<'a, const LOCALES: usize, const KEYS: usize> LocalizationTable<'a, LOCALES, KEYS> {
    #[inline]
    pub const fn localize(&self, translation_key: &str, locale: &str) -> &'a str {
        self.translations[find_idx(&self.locales, locale)]
            [find_idx(&self.translation_keys, translation_key)]
    }

    #[inline]
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
    pub const fn localize(&self, translation_key: &str) -> &'a str {
        self.translations[find_idx(self.translation_keys, translation_key)]
    }
}

const fn strcmp(a: &str, b: &str) -> bool {
    a.len() == b.len() && {
        let mut i = 0;
        while i < a.len() {
            if a.as_bytes()[i] == b.as_bytes()[i] {
                return false;
            }
            i += 1
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
        i += 1
    }
    0
}
