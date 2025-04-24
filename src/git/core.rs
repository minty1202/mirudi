use crate::git::error::GitError;
use clap::ValueEnum;
use git2::Repository;

#[cfg(test)]
use mockall::automock;

#[derive(ValueEnum, PartialEq, Clone, Debug)]
pub enum SourceKind {
    Commit,
    Worktree,
}

#[cfg_attr(test, automock)]
pub trait Provider {
    fn get_current_branch(&self) -> Result<String, GitError>;
    fn list_branches(&self) -> Result<Vec<String>, GitError>;
    fn extract_lines(
        &self,
        branch: &str,
        file_path: &str,
        start: usize,
        end: usize,
        source: SourceKind,
    ) -> Result<Vec<String>, GitError>;
    fn is_managed(&self) -> Result<bool, GitError>;
}

pub struct GitProvider;

impl GitProvider {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_lines_from_string(
        &self,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Vec<String>, GitError> {
        Ok(content
            .lines()
            .skip(start.saturating_sub(1))
            .take(end.saturating_sub(start) + 1)
            .map(|s| s.to_string())
            .collect())
    }
}

impl Provider for GitProvider {
    fn get_current_branch(&self) -> Result<String, GitError> {
        Repository::open(".")
            .map_err(|_| GitError::NotGitManaged)?
            .head()
            .map_err(|_| GitError::NotGitManaged)?
            .shorthand()
            .ok_or(GitError::EmptyBranchName)
            .map(|s| s.to_string())
    }

    fn list_branches(&self) -> Result<Vec<String>, GitError> {
        let repo = Repository::open(".").map_err(|_| GitError::NotGitManaged)?;
        let mut branches = Vec::new();
        for branch in repo.branches(None).map_err(|_| GitError::NotGitManaged)? {
            let (branch, _) = branch.map_err(|_| GitError::NotGitManaged)?;
            let name = branch
                .name()
                .map_err(|_| GitError::NotGitManaged)?
                .ok_or(GitError::EmptyBranchName)?
                .to_string();
            branches.push(name);
        }
        Ok(branches)
    }

    fn extract_lines(
        &self,
        branch: &str,
        file_path: &str,
        start: usize,
        end: usize,
        source: SourceKind,
    ) -> Result<Vec<String>, GitError> {
        match source {
            SourceKind::Worktree => {
                let content =
                    std::fs::read_to_string(file_path).map_err(|_| GitError::FileNotFound)?;
                self.extract_lines_from_string(&content, start, end)
            }
            SourceKind::Commit => {
                let repo = Repository::open(".").map_err(|_| GitError::NotGitManaged)?;
                let spec = format!("{branch}:{file_path}");
                let object = repo
                    .revparse_single(&spec)
                    .map_err(|_| GitError::FileNotFound)?;
                let blob = object.as_blob().ok_or(GitError::InvalidObjectType)?;
                let content =
                    std::str::from_utf8(blob.content()).map_err(|_| GitError::InvalidUtf8)?;
                self.extract_lines_from_string(content, start, end)
            }
        }
    }

    fn is_managed(&self) -> Result<bool, GitError> {
        Repository::open(".")
            .map(|_| true)
            .map_err(|_| GitError::NotGitManaged)
    }
}

#[cfg_attr(test, automock)]
pub trait GitOperations<T: Provider = GitProvider> {
    fn get_current_branch(&self) -> Result<String, GitError>;
    fn list_branches(&self) -> Result<Vec<String>, GitError>;
    fn extract_lines(
        &self,
        branch: &str,
        file_path: &str,
        start: usize,
        end: usize,
        source: Option<SourceKind>,
    ) -> Result<Vec<String>, GitError>;
    fn is_managed(&self) -> Result<bool, GitError>;
}

pub struct Git<T: Provider = GitProvider> {
    provider: T,
}

impl<T: Provider> Git<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }
}

impl<T: Provider> GitOperations for Git<T> {
    fn get_current_branch(&self) -> Result<String, GitError> {
        self.provider.get_current_branch()
    }

    fn list_branches(&self) -> Result<Vec<String>, GitError> {
        self.provider.list_branches()
    }

    fn extract_lines(
        &self,
        branch: &str,
        file_path: &str,
        start: usize,
        end: usize,
        source: Option<SourceKind>,
    ) -> Result<Vec<String>, GitError> {
        let source = source.unwrap_or(SourceKind::Commit);
        self.provider
            .extract_lines(branch, file_path, start, end, source)
    }

    fn is_managed(&self) -> Result<bool, GitError> {
        self.provider.is_managed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::error::GitError;
    use mockall::predicate::eq;

    mod git_get_current_branch {
        use super::*;

        #[test]
        fn test_get_current_branch() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_get_current_branch()
                .returning(|| Ok("main".to_string()));

            let git = Git::new(mock_provider);
            let result = git.get_current_branch();

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "main");
        }

        #[test]
        fn test_get_current_branch_not_git_managed() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_get_current_branch()
                .returning(|| Err(GitError::NotGitManaged));

            let git = Git::new(mock_provider);
            let result = git.get_current_branch();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GitError::NotGitManaged);
        }

        #[test]
        fn test_get_current_branch_empty_branch_name() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_get_current_branch()
                .returning(|| Err(GitError::EmptyBranchName));

            let git = Git::new(mock_provider);
            let result = git.get_current_branch();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GitError::EmptyBranchName);
        }
    }

    mod git_list_branches {
        use super::*;

        #[test]
        fn test_list_branches() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_list_branches()
                .returning(|| Ok(vec!["main".to_string(), "dev".to_string()]));

            let git = Git::new(mock_provider);
            let result = git.list_branches();

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), vec!["main", "dev"]);
        }

        #[test]
        fn test_list_branches_not_git_managed() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_list_branches()
                .returning(|| Err(GitError::NotGitManaged));

            let git = Git::new(mock_provider);
            let result = git.list_branches();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GitError::NotGitManaged);
        }
    }

    mod git_extract_lines {
        use super::*;

        #[test]
        fn test_extract_lines() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_extract_lines()
                .with(
                    eq("main"),
                    eq("file.txt"),
                    eq(1),
                    eq(10),
                    eq(SourceKind::Commit),
                )
                .returning(|_, _, _, _, _| Ok(vec!["line1".to_string(), "line2".to_string()]));

            let git = Git::new(mock_provider);
            let result = git.extract_lines("main", "file.txt", 1, 10, Some(SourceKind::Commit));

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), vec!["line1", "line2"]);
        }

        #[test]
        fn test_extract_lines_not_git_managed() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_extract_lines()
                .returning(|_, _, _, _, _| Err(GitError::NotGitManaged));

            let git = Git::new(mock_provider);
            let result = git.extract_lines("main", "file.txt", 1, 10, Some(SourceKind::Commit));

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GitError::NotGitManaged);
        }
    }

    mod is_managed {
        use super::*;

        #[test]
        fn test_is_managed() {
            let mut mock_provider = MockProvider::new();
            mock_provider.expect_is_managed().returning(|| Ok(true));

            let git = Git::new(mock_provider);
            let result = git.is_managed();

            assert!(result.is_ok());
            assert!(result.unwrap());
        }

        #[test]
        fn test_is_not_managed() {
            let mut mock_provider = MockProvider::new();
            mock_provider
                .expect_is_managed()
                .returning(|| Err(GitError::NotGitManaged));

            let git = Git::new(mock_provider);
            let result = git.is_managed();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GitError::NotGitManaged);
        }
    }
}
