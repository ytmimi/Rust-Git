use std::error::Error;
use std::path::PathBuf;
use std::{fmt, io};

pub type GitResult<T> = std::result::Result<T, GitError>;

#[derive(Debug)]
pub enum GitError {
    NotAGitRepo(PathBuf),
    Io(io::Error),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAGitRepo(p) => write!(f, "{} is not a git directory.", p.display()),
            Self::Io(err) => err.fmt(f),
        }
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> GitError {
        GitError::Io(err)
    }
}
