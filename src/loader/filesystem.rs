use std::fs::File;
use std::path::PathBuf;

use ::abc::{Loader, LoadResult, LoadError};

#[derive(Debug)]
pub struct FilesystemLoader {
    path: PathBuf,
}

impl FilesystemLoader {
    pub fn new<P>(path: P) -> Box<Self> where P: Into<PathBuf> {
        Box::new(FilesystemLoader { path: path.into() })
    }
}

impl Loader for FilesystemLoader {
    fn load(&self, name: &str) -> LoadResult {
        // TODO Real Security
        if name.contains("..") {
            return Err(LoadError::BadName(r#"".." is not supported in a path"#.to_owned()))
        }
        let mut path = self.path.join(name);
        if path.extension().is_none() { path.set_extension("tpl"); }
        info!("Path: {:?}", path);
        if !path.exists() || !path.is_file() { return Err(LoadError::NotFound); }

        let load = || -> ::std::io::Result<String> {
            use std::io::Read;
            let mut buf = String::new();
            let mut f = File::open(path)?;
            f.read_to_string(&mut buf)?;
            Ok(buf)
        };
        load().map_err(|err| LoadError::IoError(format!("{:?}", err)))
    }
}
