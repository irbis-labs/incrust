use std::fs::File;
use std::io::Read;
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
            return Err(LoadError::BadName(r#"".." is not supported in a name"#.into()))
        }
        let mut path = self.path.join(name);
        if path.extension().is_none() {
            path.set_extension("tpl");
        }
        debug!("Load template: {}, path: {:?}", name, path);
        if !path.exists() || !path.is_file() {
            return Err(LoadError::NotFound);
        }
        let mut buf = String::new();
        File::open(&path)
            .and_then(|mut f| f.read_to_string(&mut buf).map(|_| () ))
            .map_err(|err| LoadError::IoError(format!("{:?}; {:?}", err, path).into()))?;
        Ok(buf.into())
    }
}
