use std::collections::HashMap;

/// Tworzy tablicę podstawień realizującą przesunięcie cykliczne alfabetu o `n` pozycji.
///
/// # Arguments
/// * `n` - Liczba określająca kierunek i wielkość przesunięcia; wartości ujemne
///   powodują przesuwanie w lewo, dodatnie w prawo, a wynik redukowany jest modulo 26.
///
/// # Zwracana wartość
/// Zwraca mapę `HashMap<char, char>` odwzorowującą każdą wielką literę alfabetu na
/// literę powstałą po przesunięciu, gotową do użycia przez funkcję `substitute`.
pub fn generate_transposition_key(n: i16) -> HashMap<char, char> {
    let mut key = HashMap::with_capacity(26);
    let shift: u8 = ((n + 26) % 26) as u8;

    for i in 0..26 {
        let from = (b'A' + i) as char;
        let to = (b'A' + ((i + shift) % 26)) as char;
        key.insert(from, to);
    }

    key
}
