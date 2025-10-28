use std::collections::HashMap;
use std::fmt::Display;

/// Konwertuje mapę n-gramów na czytelną reprezentację tekstową z wierszami `klucz wartość`.
///
/// # Arguments
/// * `input` - Mapa zawierająca n-gram jako klucz oraz wartość możliwą do wypisania.
///
/// # Zwracana wartość
/// Zwraca łańcuch, w którym każda para wpisów formatuje się jako osobna linia, co
/// ułatwia zapis do pliku lub prezentację w konsoli.
pub fn ngram_to_string<T: Display>(input: HashMap<String, T>) -> String {
    input
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join("\n")
}
