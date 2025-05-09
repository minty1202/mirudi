use super::DiffMode;
use super::FFCommand;
use super::Range;

use crate::config::ValidatedConfigData;
use crate::diff::{Diff, DiffProvider};
use crate::git::{GitProvider, core::SourceKind};

use crate::commands::error::CommandError;

pub struct DiffHandler<'a> {
    cmd: FFCommand,
    git: &'a dyn GitProvider,
    data: ValidatedConfigData,
}

impl<'a> DiffHandler<'a> {
    pub fn build(cmd: FFCommand, git: &'a dyn GitProvider, data: ValidatedConfigData) -> Self {
        Self { cmd, git, data }
    }
}

impl DiffHandler<'_> {
    pub fn exec(&mut self) -> Result<(), CommandError> {
        self.validate_source()?;
        let old_lines = self.extract_old_lines()?;
        let new_lines = self.extract_new_lines()?;
        let diff_result = self.generate_diff(old_lines, new_lines)?;
        self.display_diff(diff_result);
        Ok(())
    }

    fn validate_source(&self) -> Result<(), CommandError> {
        let git_branch = self.git.get_current_branch()?;

        if self.cmd.source == SourceKind::Worktree && *self.data.current_branch() != git_branch {
            return Err(CommandError::InvalidInput(format!(
                "比較するブランチが現在のブランチと異なる場合、--source=worktree を指定することはできません。現在のブランチ: {}, 指定されたブランチ: {}",
                self.data.current_branch(),
                git_branch
            )));
        }
        Ok(())
    }

    fn extract_old_lines(&self) -> Result<Vec<String>, CommandError> {
        let branch = self.data.base_branch();
        let file_path = self.data.old_file_path();
        let range: Range = Range::parse(&self.cmd.old_range)?;

        let lines = self.git.extract_lines(
            branch,
            file_path,
            range.start(),
            range.end(),
            Some(self.cmd.source.clone()),
        )?;

        Ok(lines)
    }

    fn extract_new_lines(&self) -> Result<Vec<String>, CommandError> {
        let branch = self.data.current_branch();
        let file_path = self.data.new_file_path();
        let range = Range::parse(&self.cmd.new_range)?;

        let lints = self.git.extract_lines(
            branch,
            file_path,
            range.start(),
            range.end(),
            Some(self.cmd.source.clone()),
        )?;
        Ok(lints)
    }

    fn generate_diff(
        &self,
        old_lines: Vec<String>,
        new_lines: Vec<String>,
    ) -> Result<String, CommandError> {
        let diff = Diff::new(old_lines, new_lines);

        let diff_result = match self.cmd.mode {
            DiffMode::Slice => diff.slice(),
            DiffMode::Words => diff.words(),
            DiffMode::Lines => diff.lines(),
            DiffMode::Chars => diff.chars(),
        };

        Ok(diff_result)
    }

    fn display_diff(&self, diff_result: String) {
        println!(
            "\x1b[1;34m=== Diff Mode: {} ===\x1b[0m",
            self.cmd.mode.to_string().to_uppercase()
        );
        println!("{}", diff_result);
        println!("\x1b[1;34m==============================\x1b[0m\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::ff::scope_input::ScopeCommandInput;
    use crate::config::{ConfigData, ConfigScopeInput, ValidatedConfigData};
    use crate::git::GitError;
    use crate::git::core::{MockGitProvider, SourceKind};
    use mockall::predicate::eq;

    fn setup_data() -> ValidatedConfigData {
        let mut config = ConfigData::default();
        config.set_base_branch("main".to_string()).unwrap();
        config.set_scope(ConfigScopeInput {
            current_branch: Some("feature".to_string()),
            old_file_path: Some("old_file.txt".to_string()),
            new_file_path: Some("new_file.txt".to_string()),
        });
        config.try_into().unwrap()
    }

    fn setup_scope_input() -> ScopeCommandInput {
        ScopeCommandInput {
            current: false,
            branch: Some("feature".to_string()),
            old_path: Some("old_file.txt".to_string()),
            new_path: Some("new_file.txt".to_string()),
            path: None,
        }
    }

    mod validate_source {
        use super::*;

        #[test]
        fn returns_ok() {
            let mut git = MockGitProvider::new();
            let scope = setup_scope_input();
            let data = setup_data();
            let cmd = FFCommand {
                scope,
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Worktree,
                mode: DiffMode::Lines,
            };

            git.expect_get_current_branch()
                .returning(|| Ok("feature".to_string()));

            let handler = DiffHandler::build(cmd, &git, data);
            let result = handler.validate_source();
            assert!(result.is_ok());
        }

        #[test]
        fn returns_error() {
            let mut git = MockGitProvider::new();
            let scope = setup_scope_input();
            let data = setup_data();
            let cmd = FFCommand {
                scope,
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Worktree,
                mode: DiffMode::Lines,
            };

            git.expect_get_current_branch()
                .returning(|| Ok("main".to_string()));

            let handler = DiffHandler::build(cmd, &git, data);
            let result = handler.validate_source();
            assert!(result.is_err());
        }
    }

    mod extract_old_lines {
        use super::*;

        #[test]
        fn returns_vector_of_strings() {
            let mut git = MockGitProvider::new();
            let scope = setup_scope_input();
            let data = setup_data();
            let cmd = FFCommand {
                scope,
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Commit,
                mode: DiffMode::Lines,
            };

            git.expect_extract_lines()
                .with(
                    eq("main"),
                    eq("old_file.txt"),
                    eq(1),
                    eq(10),
                    eq(Some(SourceKind::Commit)),
                )
                .returning(|_, _, _, _, _| Ok(vec!["line1".to_string(), "line2".to_string()]));

            let handler = DiffHandler::build(cmd, &git, data);

            let result = handler.extract_old_lines();
            assert!(result.is_ok());
        }

        #[test]
        fn returns_error() {
            let mut git = MockGitProvider::new();
            let scope = setup_scope_input();
            let data = setup_data();
            let cmd = FFCommand {
                scope,
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Commit,
                mode: DiffMode::Lines,
            };

            git.expect_extract_lines()
                .with(
                    eq("main"),
                    eq("old_file.txt"),
                    eq(1),
                    eq(10),
                    eq(Some(SourceKind::Commit)),
                )
                .returning(|_, _, _, _, _| Err(GitError::FileNotFound));

            let handler = DiffHandler::build(cmd, &git, data);

            let result = handler.extract_old_lines();
            assert!(result.is_err());
        }
    }

    mod extract_new_lines {
        use super::*;

        #[test]
        fn returns_vector_of_strings() {
            let mut git = MockGitProvider::new();
            let scope = setup_scope_input();
            let data = setup_data();
            let cmd = FFCommand {
                scope,
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Commit,
                mode: DiffMode::Lines,
            };

            git.expect_extract_lines()
                .with(
                    eq("feature"),
                    eq("new_file.txt"),
                    eq(11),
                    eq(20),
                    eq(Some(SourceKind::Commit)),
                )
                .returning(|_, _, _, _, _| Ok(vec!["line3".to_string(), "line4".to_string()]));

            let handler = DiffHandler::build(cmd, &git, data);

            let result = handler.extract_new_lines();
            assert!(result.is_ok());
        }

        #[test]
        fn returns_error() {
            let mut git = MockGitProvider::new();
            let scope = setup_scope_input();
            let data = setup_data();
            let cmd = FFCommand {
                scope,
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Commit,
                mode: DiffMode::Lines,
            };

            git.expect_extract_lines()
                .with(
                    eq("feature"),
                    eq("new_file.txt"),
                    eq(11),
                    eq(20),
                    eq(Some(SourceKind::Commit)),
                )
                .returning(|_, _, _, _, _| Err(GitError::FileNotFound));

            let handler = DiffHandler::build(cmd, &git, data);

            let result = handler.extract_new_lines();
            assert!(result.is_err());
        }
    }

    mod generate_diff {
        use super::*;

        #[test]
        fn returns_ok() {
            let old_lines = vec!["line1".to_string(), "line2".to_string()];
            let new_lines = vec!["line3".to_string(), "line4".to_string()];
            let data = setup_data();
            let git = MockGitProvider::new();
            let cmd = FFCommand {
                scope: setup_scope_input(),
                old_range: "1-10".to_string(),
                new_range: "11-20".to_string(),
                source: SourceKind::Commit,
                mode: DiffMode::Lines,
            };

            let handler = DiffHandler::build(cmd, &git, data);
            let result = handler.generate_diff(old_lines, new_lines);
            assert!(result.is_ok());
        }
    }
}
