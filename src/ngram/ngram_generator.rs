/// Tworzy wektor wszystkich n-gramów o zadanym rozmiarze z tekstu wejściowego.
///
/// # Arguments
/// * `input` - Tekst źródłowy, z którego należy wygenerować kolejne sekwencje znaków.
/// * `ngram_size` - Rozmiar n-gramu określający liczbę znaków w każdej sekwencji.
///
/// # Zwracana wartość
/// Zwraca wektor łańcuchów, w którym każda pozycja odpowiada kolejnemu n-gramowi
/// wyodrębnionemu z tekstu wejściowego.
pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}
