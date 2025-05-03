use crate::git::error::GitError;
use clap::ValueEnum;
use git2::{DiffOptions, Repository};
use std::path::PathBuf;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
struct DiffEntry {
    path: PathBuf,
    depth: usize,
}

#[derive(ValueEnum, PartialEq, Clone, Debug)]
pub enum SourceKind {
    Commit,
    Worktree,
}

#[cfg_attr(test, automock)]
pub trait GitProvider {
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
    fn list_changed_files(
        &self,
        base_branch: &str,
        target_branch: &str,
    ) -> Result<Vec<String>, GitError>;
    fn is_managed(&self) -> Result<bool, GitError>;
}

pub struct Git;

impl Git {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_lines_from_string(
        &self,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Vec<String>, GitError> {
        let start_index = start.max(1);

        if end < start_index {
            return Ok(Vec::new());
        }

        let count = end - start_index + 1;

        Ok(content
            .lines()
            .skip(start_index - 1)
            .take(count)
            .map(|s| s.to_string())
            .collect())
    }
}

impl GitProvider for Git {
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
        source: Option<SourceKind>,
    ) -> Result<Vec<String>, GitError> {
        let source = source.unwrap_or(SourceKind::Commit);
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

    fn list_changed_files(
        &self,
        base_branch: &str,
        target_branch: &str,
    ) -> Result<Vec<String>, GitError> {
        let repo = Repository::open(".").map_err(|_| GitError::NotGitManaged)?;

        let base_object = repo
            .revparse_single(base_branch)
            .map_err(|_| GitError::FileNotFound)?;
        let target_object = repo
            .revparse_single(target_branch)
            .map_err(|_| GitError::FileNotFound)?;

        let base_tree = base_object
            .peel_to_tree()
            .map_err(|_| GitError::InvalidObjectType)?;
        let target_tree = target_object
            .peel_to_tree()
            .map_err(|_| GitError::InvalidObjectType)?;

        let mut diff_opts = DiffOptions::new();
        let diff = repo
            .diff_tree_to_tree(Some(&base_tree), Some(&target_tree), Some(&mut diff_opts))
            .map_err(|_| GitError::DiffExtractionFailed)?;

        let mut entries = Vec::new();

        diff.foreach(
            &mut |delta, _| {
                let status = delta.status();
                let path = match status {
                    git2::Delta::Deleted => delta.old_file().path(),
                    _ => delta.new_file().path().or_else(|| delta.old_file().path()),
                };

                if let Some(path) = path {
                    let path = path.to_path_buf();
                    let depth = path.components().count().saturating_sub(1);

                    entries.push(DiffEntry { path, depth });
                }
                true
            },
            None,
            None,
            None,
        )
        .map_err(|_| GitError::DiffExtractionFailed)?;

        entries.sort_by(|a, b| {
            let a_parts: Vec<_> = a.path.components().collect();
            let b_parts: Vec<_> = b.path.components().collect();

            for (a_part, b_part) in a_parts.iter().zip(b_parts.iter()) {
                match a_part.cmp(b_part) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }

            a.depth.cmp(&b.depth)
        });

        Ok(entries
            .into_iter()
            .map(|e| e.path.to_string_lossy().to_string())
            .collect())
    }

    fn is_managed(&self) -> Result<bool, GitError> {
        Repository::open(".")
            .map(|_| true)
            .map_err(|_| GitError::NotGitManaged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_range() {
        let git = Git::new();

        let text = "foo\nbar\nbaz\nqux";

        let result = git
            .extract_lines_from_string(text, /*start=*/ 2, /*end=*/ 3)
            .unwrap();

        assert_eq!(result, vec!["bar".to_string(), "baz".to_string()]);
    }

    #[test]
    fn start_zero_and_saturating() {
        let git = Git::new();
        let text = "a\nb\nc";
        let result = git.extract_lines_from_string(text, 0, 2).unwrap();
        assert_eq!(result, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn end_less_than_start() {
        let git = Git::new();
        let text = "x\ny\nz";
        let result = git.extract_lines_from_string(text, 3, 1).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn out_of_bounds_truncate() {
        let git = Git::new();
        let text = "one\ntwo";
        let result = git.extract_lines_from_string(text, 2, 100).unwrap();
        assert_eq!(result, vec!["two".to_string()]);
    }

    #[test]
    fn empty_content() {
        let git = Git::new();
        let result = git.extract_lines_from_string("", 1, 5).unwrap();
        assert!(result.is_empty());
    }
}
