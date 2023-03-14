use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use futures::{future::BoxFuture, FutureExt, TryFutureExt};

use super::FileSource;

impl<T, E> FileSource for T
where
    T: Fn(Targets) -> BoxFuture<'static, Result<Vec<u8>, E>> + Send,
    E: std::error::Error + 'static + Send,
{
    fn obtain_content(
        &self,
        targets: Targets,
    ) -> BoxFuture<'static, Result<Vec<u8>, Box<dyn std::error::Error + Send>>> {
        self(targets)
            .map_err(|error| -> Box<dyn std::error::Error + Send> { Box::new(error) })
            .boxed()
    }
}

#[derive(Debug, Clone)]
pub struct Targets {
    current: PathBuf,
    all: BTreeSet<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
#[error("target not found: {target}")]
pub struct TargetNotFoundError {
    target: PathBuf,
}

impl TargetNotFoundError {
    pub fn new(target: PathBuf) -> Self {
        Self { target }
    }
}

impl Targets {
    pub(crate) fn new(current: PathBuf, all: BTreeSet<PathBuf>) -> Self {
        Self { current, all }
    }

    pub fn path_of(&self, path: impl AsRef<Path>) -> Result<String, TargetNotFoundError> {
        let path = path.as_ref();

        assert!(path.is_absolute(), "path not absolute: {path:?}");

        self.all
            .contains(path)
            .then(|| {
                PathBuf::from_iter([PathBuf::from("/"), path.to_owned()])
                    .to_str()
                    .map(|path| path.to_owned())
            })
            .flatten()
            .map(|path| {
                if path == "/index.html" {
                    String::from("/")
                } else {
                    path
                }
            })
            .ok_or_else(|| TargetNotFoundError::new(path.to_owned()))
    }

    pub fn current_path(&self) -> String {
        self.path_of(&self.current).unwrap()
    }
}
