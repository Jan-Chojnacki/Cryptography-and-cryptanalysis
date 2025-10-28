use std::collections::HashMap;

/// Podstawia każdy znak wejściowy zgodnie z dostarczonym odwzorowaniem klucza.
///
/// # Arguments
/// * `input` - Tekst do przetworzenia, zawierający znaki odpowiadające kluczowi.
/// * `key` - Mapa podstawień opisująca odwzorowanie znaków wejściowych na wyjściowe.
///
/// # Zwracana wartość
/// Zwraca nowy łańcuch znaków powstały poprzez kolejne zastępowanie liter zgodnie
/// z dostarczonym kluczem. Funkcja zakłada kompletność mapowania (`unwrap`).
pub fn substitute(input: &str, key: &HashMap<char, char>) -> String {
    input.chars().map(|x| key.get(&x).unwrap()).collect()
}
