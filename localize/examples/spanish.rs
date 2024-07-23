use localize::localization_table;

localization_table! {Spanglish = LDSL {
    "greeting" = {
        en => "Hello",
        es => "Hola"
    },
    "farewell" = {
        en => "Goodbye",
        es => "Adiós"
    }
}}

fn main() {
    println!(
        "Greeting in English: {}",
        Spanglish::localize("greeting", "en")
    );
    println!(
        "Greeting in Spanish: {}",
        Spanglish::localize("greeting", "es")
    );
    println!(
        "Farewell in English: {}",
        Spanglish::localize("farewell", "en")
    );
    println!(
        "Farewell in Spanish: {}",
        Spanglish::localize("farewell", "es")
    );
}
