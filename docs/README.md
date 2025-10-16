# Kryptografia i kryptoanaliza
## Laboratorium 1
### Grupa 1ID24B
### Autorzy: Jakub Babiarski, Jan Chojnacki

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
fn main() {
    let args = Args::parse();

    let input = Path::new(&args.input);
    if input.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }

    let output = Path::new(&args.output);
    if output.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }

    let key = Path::new(&args.key);
    if key.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }

    let operating_mode = match (args.mode_group.decrypt, args.mode_group.encrypt) {
        (true, false) => OperatingMode::DECRYPTION,
        (false, true) => OperatingMode::ENCRYPTION,
        (_, _) => panic!("Only one operating mode can be selected at a time.")
    };

    let input = OpenOptions::new()
        .read(true)
        .open(input)
        .expect("Failed to open input file");

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .expect("Failed to open output file");

    let key = OpenOptions::new()
        .read(true)
        .open(key)
        .expect("Failed to open key file");

    let input = input_parser(input);
    let key = key_parser(key, operating_mode);

    output
        .write_all(input.as_bytes())
        .expect(format!("Could not write to output file at: {:?}.", output).as_str());
}
```

Kod źródłowy zawiera główne składowe programu
- co jest wejściem funkcji
- co jest wyjściem funkcji
- co implementuje dana funkcja

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