use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Buduje mapowanie znaków na podstawie pliku z kluczem, uwzględniając kierunek konwersji.
///
/// # Arguments
/// * `key` - Uchwyt do pliku zawierającego pary znaków opisujące odwzorowanie klucza.
/// * `decryption` - Flaga określająca kierunek mapowania; `true` oznacza odwrócenie par na
///   potrzeby deszyfrowania.
///
/// # Zwracana wartość
/// Zwraca kompletną mapę `HashMap<char, char>` zapewniającą bijekcję pomiędzy znakami alfabetu.
pub fn key_parser(key: File, decryption: bool) -> HashMap<char, char> {
    let mut map: HashMap<char, char> = HashMap::new();
    let reader = BufReader::new(key);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Invalid key format.")
            }
            match decryption {
                false => {
                    let key = parts[0].chars().next().unwrap();
                    let value = parts[1].chars().next().unwrap();
                    map.insert(key, value);
                }
                true => {
                    let key = parts[1].chars().next().unwrap();
                    let value = parts[0].chars().next().unwrap();
                    map.insert(key, value);
                }
            }
        }
    }

    let key_test: HashSet<char> = map.iter().map(|(&k, _)| k).collect();
    let value_test: HashSet<char> = map.iter().map(|(_, &v)| v).collect();

    if key_test.len() != 26 || value_test.len() != 26 {
        panic!("Invalid key values.")
    }

    map
}
