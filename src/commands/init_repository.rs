use std::{env, fs};
use std::io::Write;

use crate::GitResult;
use crate::core::repository::Repository;

/// Initialize an empty git repository in the current directory.
///
/// Create the following directories:
/// * ``.git/refs/heads``
/// * ``.git/refs/tags``
/// * ``.git/objects/info``
/// * ``.git/objects/pack``
///
/// Create the following files:
/// * ``.git/HEAD``
/// * ``.git/description``
/// * ``.git/config``
pub fn initialize_git_repository() -> GitResult<()> {
    let cwd = env::current_dir()?;
    let repo = Repository::maybe_uninitialized_repo(cwd);

    fs::create_dir_all(repo.heads())?;
    fs::create_dir_all(repo.tags())?;
    fs::create_dir_all(repo.info())?;
    fs::create_dir_all(repo.pack())?;

    let head = repo.HEAD();
    if !head.exists() {
        let mut file = fs::File::create(head)?;
        file.write_all(b"ref: refs/heads/main")?;
    }

    let description = repo.description();
    if !description.exists() {
        let mut file = fs::File::create(description)?;
        let message =
            b"Unnamed repository; edit this file 'description' to name the repository.\n";
        file.write_all(message)?;
    }

    // TODO: Write initial configuration options to file
    let config = repo.config();
    if !config.exists() {
        let _ = fs::File::create(config)?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::env;
    use crate::GitResult;
    use tempfile::TempDir;

    #[allow(non_snake_case)]
    #[test]
    fn create_empty_git_repositry() -> GitResult<()> {
        // create a temporary directory and set the current directory there
        let tmp_dir = TempDir::new().unwrap();
        env::set_current_dir(tmp_dir.path())?;

        let heads = tmp_dir.path().join(".git/refs/heads");
        let tags = tmp_dir.path().join(".git/refs/heads");
        let info = tmp_dir.path().join(".git/objects/info");
        let pack = tmp_dir.path().join(".git/objects/pack");
        let HEAD = tmp_dir.path().join(".git/HEAD");
        let description = tmp_dir.path().join(".git/description");
        let config = tmp_dir.path().join(".git/config");

        // None of these paths exist before we initialize the repository
        assert!(!heads.exists());
        assert!(!tags.exists());
        assert!(!info.exists());
        assert!(!pack.exists());
        assert!(!HEAD.exists());
        assert!(!description.exists());
        assert!(!config.exists());

        // initialize the repository
        super::initialize_git_repository()?;

        // all of these paths exist now that the repo has been initialized
        assert!(heads.exists());
        assert!(tags.exists());
        assert!(info.exists());
        assert!(pack.exists());
        assert!(HEAD.exists());
        assert!(description.exists());
        assert!(config.exists());
        Ok(())
    }
}