
//! Taken from: https://github.com/rust-lang/cargo/blob/5fe8ab57e2a88ccaaab0821c306203eb19edf8fd/src/cargo/util/restricted_names.rs

pub static KEYWORDS: &'static [&'static str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

pub static WINDOWS: &'static [&'static str] = &[
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

pub static ARTIFACTS: &'static [&'static str] = &["deps", "examples", "build", "incremental"];

pub fn in_keywords(s: impl AsRef<str>) -> bool {
    KEYWORDS.contains(&s.as_ref())
}

pub fn in_windows(s: impl AsRef<str>) -> bool {
    WINDOWS.contains(&s.as_ref())
}

pub fn in_artifacts(s: impl AsRef<str>) -> bool {
    ARTIFACTS.contains(&s.as_ref())
}

#[derive(Debug)]
pub enum Reservation {
    Keywords,
    Windows,
    Artifacts,
}

pub fn is_reserved(s: impl AsRef<str>) -> Result<(), Reservation> {
    let s = s.as_ref();
    if in_keywords(s) {
        Err(Reservation::Keywords)
    } else if in in_windows(s) {
        Err(Reservation::Windows)
    } else if in_artifacts(s) {
        Err(Reservation::Artifacts)
    } else {
        Ok(())
    }
}
