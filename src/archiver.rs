use std::path::PathBuf;

pub trait Archiver {
    fn execute();
    fn format();
}

pub struct ArchiverOpts {
    pub Output: PathBuf,
    pub targets: Vec<PathBuf>,
    pub Recursive: bool,
    pub Overwrite: bool,
}
