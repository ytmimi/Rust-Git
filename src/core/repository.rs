use std::path::{Path, PathBuf};
use std::env;

use crate::{GitError, GitResult};

/// A Git Repository
pub struct Repository {
    base_dir: PathBuf,
}

impl Repository {
    /// Construct a Repository object from a directory that may not already be a Git Repository.
    ///
    /// This function is expected to be used when creating a new Git repository.
    pub fn maybe_uninitialized_repo<P: AsRef<Path>>(path: P) -> Self {
        Self {
            base_dir: path.as_ref().to_path_buf(),
        }
    }

    /// Construct a Repository object from the current working directory or a parent directory.
    ///
    /// This function is expected to be used when performing operations on an already
    /// initialized Git repository.
    /// If a .git/ subdirectory is not found within the current working directory or any
    /// of its parent directories, then a GitError is returned.
    pub fn from_cwd_or_parent() -> GitResult<Self> {
        let path = env::current_dir()?;
        let base_dir = find_repo(path)?;
        // base_dir is guarunteed to have a .git/ directory
        Ok(Self::maybe_uninitialized_repo(base_dir))
    }

    /// Returns the path to the local config file.
    ///
    /// From the root of the Git repository this file is located at .git/config
    /// See the [Getting Started][1] guide for details on the local configuration file's location.
    /// [1]: https://git-scm.com/book/en/v2/Getting-Started-First-Time-Git-Setup
    pub fn config(&self) -> PathBuf {
        self.base_dir.join(".git/config")
    }

    /// Returns the Path to the repository's description file.
    ///
    /// From the root of the Git repository this file is located at .git/description
    pub fn description(&self) -> PathBuf {
        self.base_dir.join(".git/description")
    }

    /// Returns the Path to the repository's HEAD file.
    ///
    /// From the root of the Git repository this file is located at .git/HEAD
    #[allow(non_snake_case)]
    pub fn HEAD(&self) -> PathBuf {
        self.base_dir.join(".git/HEAD")
    }

    /// Returns the Path to the repository's refs directory.
    ///
    /// From the root of the Git repository this directory is located at .git/refs/
    pub fn refs(&self) -> PathBuf {
        self.base_dir.join(".git/refs")
    }

    /// Returns the Path to the repository's heads directory.
    ///
    /// From the root of the Git repository this directory is located at .git/refs/heads/
    pub fn heads(&self) -> PathBuf {
        self.base_dir.join(".git/refs/heads")
    }

    /// Returns the Path to the repository's tags directory.
    ///
    /// From the root of the Git repository this directory is located at .git/refs/tags/
    pub fn tags(&self) -> PathBuf {
        self.base_dir.join(".git/refs/tags")
    }

    /// Returns the Path to the repository's objects directory.
    ///
    /// From the root of the Git repository this directory is located at .git/objects/
    pub fn objects(&self) -> PathBuf {
        self.base_dir.join(".git/objects")
    }

    /// Returns the Path to the repository's info directory.
    ///
    /// From the root of the Git repository this directory is located at .git/objects/info
    pub fn info(&self) -> PathBuf {
        self.base_dir.join(".git/objects/info")
    }

    /// Returns the Path to the repository's pack directory.
    ///
    /// From the root of the Git repository this directory is located at .git/objects/pack
    pub fn pack(&self) -> PathBuf {
        self.base_dir.join(".git/objects/pack")
    }
}

/// Check a directory and all of its parent directories for a .git/ sub-directory.
///
/// If no .git/ directory is found, then a GitError::NotAGitRepo is returned.
/// Otherwise, the path to the Git repositories parent directory is returned.
fn find_repo<P: AsRef<Path>>(path: P) -> GitResult<PathBuf> {
    for p in path.as_ref().ancestors() {
        if contains_git_dir(p) {
            return Ok(p.to_path_buf());
        }
    }
    Err(GitError::NotAGitRepo(path.as_ref().to_path_buf()))
}

/// Check if a directory contains a .git/ sub-directory
fn contains_git_dir<P: AsRef<Path>>(path: P) -> bool {
    if !path.as_ref().is_dir() {
        return false;
    }

    path.as_ref().join(".git").exists()
}

