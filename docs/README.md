# Kryptografia i kryptoanaliza
## Laboratorium 1
### Grupa 1ID24B
### Autorzy: Jakub Babiarski, Jan Chojnacki

``` mermaid
flowchart TD
    A[Wejście]
    B(Odczytywanie i walidacja poprawności kombinacji argumentów)
    C(Walidacja ścieżek przekazanych w argumentach)
    D{Określenie trybu pracy programu}
    F(Koniec)

    S1(Otwarcie plików przekazanych w argumentach)
    S2(Przetworzenie zawartości wejścia i klucza)
    S3(Zaszyfrowanie wejścia na podstawie klucza)
    S4(Zapisanie zaszyfrowanego tekstu do pliku wyjścia)

    D1(Otwarcie plików przekazanych w argumentach)
    D2(Przetworzenie zawartości wejścia i klucza)
    D3(Odszyfrowanie wejścia na podstawie klucza)
    D4(Zapisanie odszyfrowanego tekstu do pliku wyjścia)

    G1(Otwarcie plików przekazanych w argumentach)
    G2(Przetworzenie zawartości wejścia)
    G3(Wygenerowanie ngramu)
    G4(Wypisanie ngramu na standardowe wyjście)
    G5(Zapisanie ngramu do pliku wyjścia)

    O1(Otwarcie plików przekazanych w argumentach)
    O2(Przetworzenie zawartości wejścia)
    O3(Obliczenie prawdopodobieństwa wystąpienia n-gramu)
    O4(Wypisanie ngramu na standardowe wyjście)

    T1(Otwarcie plików przekazanych w argumentach)
    T2(Przetworzenie zawartości wejścia)
    T3(Wygenerowanie ngramu dla wejścia)
    T4(Obliczenie prawdopodobieństwa wystąpienia n-gramu dla ngramu referencyjnego)
    T5(Obliczenie wyniku testu)
    T6(Wypisanie wyniku testu na standardowe wyjście)

    A --> B
    B --> C
    C --> D

    D --> |Szyfrowanie| S1 --> S2 --> S3 --> S4 --> F
    D --> |Deszyfrowanie| D1 --> D2 --> D3 --> D4 --> F
    D --> |Generowanie n-gramu| G1 --> G2 --> G3 --> G4 --> G5 --> F
    D --> |Odczytywanie n-gramu| O1 --> O2 --> O3 --> O4 --> F
    D --> |Test x^2| T1 --> T2 --> T3 --> T4 --> T5 --> T6 --> F
```

### Zadanie 1

Korzystając z języka Rust, dokonaj implementacji programu szyfrującego i deszyfrującego zadany tekst.
1. Tekst jawny powinien być importowany do programu z pliku tekstowego, którego nazwa określona powinna być
   po zdefiniowanym argumencie / fladze: -i. 
2. Wynik pracy programu powinien być eksportowany do pliku tekstowego, którego nazwa określona powinna być
   po zdefiniowanym argumencie / fladze: -o.
3. Klucz powinien być importowany z pliku tekstowego, którego nazwa powinna być określona po zdefiniowanym
   argumencie / fladze: -k.
4.  Tryb pracy programu powinien być określony poprzez flagi: -e dla procesu szyfrowania, -d dla procesu deszyfrowania
    
#### Implementacja

Kod źródłowy pliku main.rs

``` Rust
mod args;
mod converters;
mod file_handling;
mod file_parsers;
mod generators;
mod operating_mode;
mod operations;

use crate::args::Args;
use crate::operating_mode::OperatingMode;
use clap::Parser;

/// Entrypoint that parses CLI arguments, validates them and dispatches the
/// selected operating mode.
fn main() {
    let args = Args::parse();
    args.validate().expect("Validation failed");

    let operating_mode = args.operating_mode();

    match operating_mode {
        OperatingMode::Encryption => {
            operations::encryption_decryption(args, operating_mode);
        }
        OperatingMode::Decryption => {
            operations::encryption_decryption(args, operating_mode);
        }
        OperatingMode::NgramGenerator => {
            operations::ngram_generator(args);
        }
        OperatingMode::NgramReader => {
            operations::ngram_reader(args);
        }
        OperatingMode::X2Test => {
            operations::x2test(args);
        }
    }
}

```

Kod źródłowy pliku main.rs zawiera jedną funckję main()
- funkcja nie przyjmuje argumentów wejścia
- funkcja nie zwraca żadnych wartości
- funkcja implementuje działanie całego programu zależnie od wybranych przez użytownika flag

Kod źródłowy pliku main.rs

``` Rust
use crate::operating_mode::OperatingMode;
use crate::operating_mode::OperatingMode::{
    Decryption, Encryption, NgramGenerator, NgramReader, X2Test,
};
use clap::ArgGroup;
use std::path::PathBuf;

/// Command line arguments accepted by the application.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group = ArgGroup::new("mode").args(["encrypt", "decrypt", "gram", "read_ngram"]))]
#[command(group = ArgGroup::new("ngram-file").args(["gram", "read_ngram"]))]
pub struct Args {
    /// Path to input file.
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,
    /// Path to output file.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    /// Path to key file.
    #[arg(short, long, value_name = "FILE")]
    pub key: Option<PathBuf>,
    /// Program operation mode.
    #[clap(flatten)]
    pub mode_group: ModeGroup,

    /// Path to ngram file.
    #[arg(value_name = "FILE", requires = "ngram-file")]
    pub ngram_file: Option<PathBuf>,
}

/// Flags that identify the operating mode in which the application should run.
#[derive(clap::Args, Debug)]
pub struct ModeGroup {
    /// Encryption mode.
    #[arg(short, long, requires_all = ["input", "output", "key"])]
    pub encrypt: bool,
    /// Decryption mode.
    #[arg(short, long, requires_all = ["input", "output", "key"])]
    pub decrypt: bool,
    /// Ngram generation mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4), requires_all = ["input"]
    )]
    pub gram: Option<u8>,
    /// Ngram reading mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
    pub read_ngram: Option<u8>,
    /// Generating x^2 test.
    #[arg(short, requires_all = ["read_ngram", "input"])]
    pub s: bool,
}

impl Args {
    /// Performs basic validation of the supplied paths and flags.
    pub fn validate(&self) -> Result<(), String> {
        // Ensure that each path argument uses a supported file extension.
        self.validate_paths()?;

        Ok(())
    }

    /// Determines which operating mode should be executed based on the provided flags.
    pub fn operating_mode(&self) -> OperatingMode {
        // Translate the clap-generated booleans into simple aliases for readability.
        let e = self.mode_group.encrypt;
        let d = self.mode_group.decrypt;
        let n = self.mode_group.gram.is_some();
        let r = self.mode_group.read_ngram.is_some();
        let s = self.mode_group.s;

        // Map the mutually exclusive combinations of flags to their semantic meaning.
        match (e, d, n, r, s) {
            (true, false, false, false, false) => Encryption,
            (false, true, false, false, false) => Decryption,
            (false, false, true, false, false) => NgramGenerator,
            (false, false, false, true, false) => NgramReader,
            (false, false, false, true, true) => X2Test,
            _ => panic!("Impossible combination of flags"),
        }
    }

    /// Validates that every provided path points to a `.txt` file.
    fn validate_paths(&self) -> Result<(), String> {
        if let Some(input) = &self.input {
            if input.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                return Err("Only files with .txt extension are supported.".into());
            }
        }

        if let Some(output) = &self.output {
            if output.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                return Err("Only files with .txt extension are supported.".into());
            }
        }

        if let Some(key) = &self.key {
            if key.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                return Err("Only files with .txt extension are supported.".into());
            }
        }

        Ok(())
    }
}

```

Kod zawiera 2 struktury i 3 funkcje <br>
Struktura Args{}
-Przechowuje dane przekazywane przy wywołaniu programu. Dane to: ścieżka do pliku wejściowego, ścieżka zapisu pliku wyjściowego, plik klucza oraz flagi sterujące działaniem programu.
-Sprawdza czy ścieżki do plików podane w argumencie zawierają wspierane rozszerzenia plików (validate() i validate_paths())
-
#### Wyniki

W tej sekcji powinny być przedstawione wyniki pracy programu

``` sh
RESULT
```

Wyniki powinny być zinterpretowane.

### Zadanie 2

Rozbudować program z poprzedniego przykładu poprzez dodanie do niego funkcjonalności generowania statystyk licz-
ności występowania n-gramów (sekwencji kolejnych liter), to jest mono-gramów (pojedynczych liter), bi-gramów (wy-
razów dwuliterowych), tri-gramów (wyrazów trzyliterowych) oraz quad-gramów (wyrazów czteroliterowych). Funk-
cjonalność ta powinna być wyzwalana poprzez dodanie do programu jednej z następujących flag: -g1, -g2, -g3 lub
-g4, po której powinna zostać określona nazwa pliku, do którego zapisane zostaną wyniki.

#### Implementacja

Implementacja powinna przedstawiać kod źródłowy programu.

``` Rust
fn main() {
    println!("Hello, world!");
}
```

Kod źródłowy powinien być podzielony na części (definicje i funkcje). Każdy fragment programu powinien być opisany:
- co jest wejściem funkcji
- co jest wyjściem funkcji
- co implementuje dana funkcja

#### Wyniki

W tej sekcji powinny być przedstawione wyniki pracy programu

``` sh
RESULT
```

Wyniki powinny być zinterpretowane.

### Zadanie 3

#### Implementacja

Implementacja powinna przedstawiać kod źródłowy programu.

``` Rust
fn main() {
    println!("Hello, world!");
}
```

Kod źródłowy powinien być podzielony na części (definicje i funkcje). Każdy fragment programu powinien być opisany:
- co jest wejściem funkcji
- co jest wyjściem funkcji
- co implementuje dana funkcja

#### Wyniki

W tej sekcji powinny być przedstawione wyniki pracy programu

``` sh
RESULT
```

Wyniki powinny być zinterpretowane.

### Zadanie 4

#### Implementacja

Implementacja powinna przedstawiać kod źródłowy programu.

``` Rust
fn main() {
    println!("Hello, world!");
}
```

Kod źródłowy powinien być podzielony na części (definicje i funkcje). Każdy fragment programu powinien być opisany:
- co jest wejściem funkcji
- co jest wyjściem funkcji
- co implementuje dana funkcja

#### Wyniki

W tej sekcji powinny być przedstawione wyniki pracy programu

``` sh
RESULT
```

Wyniki powinny być zinterpretowane.
