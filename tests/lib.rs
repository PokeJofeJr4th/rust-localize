use localize_macros::localization_table;

localization_table! {TestLocTable = LDSL {
    "_" = {
        en => "<Unknown Translation>",
        es => "<No Savo>",
    },
    greeting = {
        en => "Hello",
        es => "Hola"
    },
    apple = {
        en => "Apple",
        fr => "Pomme"
    }
}}

/// Make sure the localized strings returned by the function are correct
#[test]
fn test_table_localize() {
    assert_eq!(TestLocTable::localize("greeting", "en"), "Hello");
    assert_eq!(TestLocTable::localize("greeting", "es"), "Hola");
    assert_eq!(TestLocTable::localize("apple", "en"), "Apple");
    assert_eq!(TestLocTable::localize("apple", "fr"), "Pomme");
}

/// Make sure the fallback strings returned by the function are correct
#[test]
fn test_table_localize_missing() {
    assert_eq!(TestLocTable::localize("apple", "es"), "<No Savo>");
    assert_eq!(
        TestLocTable::localize("farewell", "en"),
        "<Unknown Translation>"
    );
}

/// Make sure the `const` locale variables are set up properly
#[test]
fn test_const_locale() {
    assert_eq!(TestLocTable::EN.localize("greeting"), "Hello");
    assert_eq!(TestLocTable::EN.localize("apple"), "Apple");
    assert_eq!(TestLocTable::ES.localize("greeting"), "Hola");
    assert_eq!(TestLocTable::FR.localize("apple"), "Pomme");
}

/// Make sure the `get_locale` function works
#[test]
fn test_get_locale() {
    let en = TestLocTable::get_locale("en");
    assert_eq!(format!("{en}"), "en");
    assert_eq!(en.localize("greeting"), "Hello");
    assert_eq!(en.localize("apple"), "Apple");

    let fr = TestLocTable::get_locale("fr");
    assert_eq!(format!("{fr}"), "fr");
    assert_eq!(fr.localize("apple"), "Pomme");

    let es = TestLocTable::get_locale("es");
    assert_eq!(format!("{es}"), "es");
    assert_eq!(es.localize("greeting"), "Hola");
}
