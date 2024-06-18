#[derive(Debug, PartialEq)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    TarZstd,
    SevenZ,
    LHA,
    Rar,
    Unknown(String),
}
