use clap::Subcommand;
use std::path::PathBuf;

/// Parametry wiersza poleceń kontrolujące działanie aplikacji.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Główne polecenie wybierające tryb pracy narzędzia.
    #[command(subcommand)]
    pub commands: Commands,
}

/// Dostępne polecenia główne aplikacji.
#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Commands {
    /// Szyfrowanie tekstu za pomocą wybranego algorytmu klasycznego.
    Encrypt {
        /// Podpolecenie określające algorytm i jego parametry.
        #[command(subcommand)]
        algorithm_command: AlgorithmCommand,
    },
    /// Deszyfrowanie tekstu za pomocą wybranego algorytmu klasycznego.
    Decrypt {
        /// Podpolecenie określające algorytm i jego parametry.
        #[command(subcommand)]
        algorithm_command: AlgorithmCommand,
    },
    /// Operacje na statystykach n-gramowych.
    Ngram {
        /// Podpolecenie definiujące rodzaj operacji na n-gramach.
        #[command(subcommand)]
        ngram_command: NgramCommand,
    },
    /// Uruchomienie ataków kryptograficznych typu brute force.
    Attack {
        /// Podpolecenie wskazujące atakowany algorytm i dane pomocnicze.
        #[command(subcommand)]
        attack_command: AttackCommand,
    },
    /// Porównanie podobieństwa rozkładu tekstu wejściowego z referencyjnym.
    Similarity {
        /// Rozmiar n-gramów wykorzystywanych w analizie statystycznej.
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        /// Ścieżka do pliku z referencyjnymi częstotliwościami n-gramów.
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Ścieżka do pliku zawierającego analizowany tekst.
        #[arg(short, long)]
        input: PathBuf,
    },
}

/// Polecenia dotyczące generowania i odczytu n-gramów.
#[derive(Subcommand, Debug)]
pub enum NgramCommand {
    /// Generuje histogram n-gramów na podstawie wejściowego tekstu.
    Generate {
        /// Rozmiar n-gramów do wygenerowania.
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
        g: u8,
        /// Plik z tekstem źródłowym wykorzystywanym do analizy.
        #[arg(short, long)]
        input: PathBuf,
        /// Opcjonalny plik wyjściowy na zapisane statystyki n-gramów.
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
    },
    /// Wyświetla referencyjny zbiór n-gramów z pliku.
    Read {
        /// Rozmiar n-gramów odpowiadający plikowi referencyjnemu.
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        /// Ścieżka do pliku z zapisanymi częstotliwościami n-gramów.
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

/// Wybór algorytmów szyfrowania i ich parametrów.
#[derive(Subcommand, Debug)]
pub enum AlgorithmCommand {
    /// Algorytm podstawieniowy z kluczem dostarczonym w pliku.
    Substitution {
        /// Plik z tekstem jawnym lub zaszyfrowanym.
        #[arg(short, long)]
        input: PathBuf,
        /// Plik wyjściowy na wynik szyfrowania bądź deszyfrowania.
        #[arg(short, long)]
        output: PathBuf,
        /// Plik z mapowaniem znaków stanowiącym klucz podstawienia.
        #[arg(short, long)]
        key: PathBuf,
    },
    /// Algorytm przestawieniowy z przesunięciem klucza.
    Transposition {
        /// Plik z tekstem jawnym lub zaszyfrowanym.
        #[arg(short, long)]
        input: PathBuf,
        /// Plik wyjściowy na wynik szyfrowania bądź deszyfrowania.
        #[arg(short, long)]
        output: PathBuf,
        /// Przesunięcie klucza w zakresie od 1 do 25.
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
        key: u8,
    },
    /// Algorytm afiniczny z parametrami multiplikatywnym i addytywnym.
    Affine {
        /// Plik z tekstem jawnym lub zaszyfrowanym.
        #[arg(short, long)]
        input: PathBuf,
        /// Plik wyjściowy na wynik szyfrowania bądź deszyfrowania.
        #[arg(short, long)]
        output: PathBuf,
        /// Współczynnik multiplikatywny `a`, względnie pierwszy z 26.
        #[arg(short, long)]
        a: u32,
        /// Współczynnik addytywny `b` w zakresie alfabetu.
        #[arg(short, long)]
        b: u32,
    },
}

/// Polecenia ataków kryptograficznych.
#[derive(Subcommand, Debug)]
pub enum AttackCommand {
    /// Ataki typu brute force na obsługiwane algorytmy.
    BruteForce {
        /// Wybrany algorytm oraz parametry niezbędne do przeprowadzenia ataku.
        #[command(subcommand)]
        algorithm: AttackAlgorithmCommand,
    },
}

/// Warianty atakowanych algorytmów brute force.
#[derive(Subcommand, Debug)]
pub enum AttackAlgorithmCommand {
    /// Przeszukiwanie kluczy dla szyfru przestawieniowego.
    Transposition {
        /// Argumenty wspólne dla ataków, zawierające pliki wejścia i referencję.
        #[clap(flatten)]
        args: AttackArgs,
    },
    /// Przeszukiwanie kluczy dla szyfru afinicznego.
    Affine {
        /// Argumenty wspólne dla ataków, zawierające pliki wejścia i referencję.
        #[clap(flatten)]
        args: AttackArgs,
    },
}

/// Wspólne parametry uruchomienia ataków brute force.
#[derive(clap::Args, Debug)]
pub struct AttackArgs {
    /// Plik z tekstem zaszyfrowanym będącym celem ataku.
    #[arg(short, long)]
    pub input: PathBuf,
    /// Plik, do którego zostanie zapisane najlepsze odgadnięte odszyfrowanie.
    #[arg(short, long)]
    pub output: PathBuf,
    /// Rozmiar n-gramów wykorzystywanych w teście statystycznym.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
    pub r: u8,
    /// Plik z referencyjnymi częstotliwościami n-gramów wykorzystywanymi w ocenie.
    #[arg(value_name = "FILE")]
    pub file: PathBuf,
}
