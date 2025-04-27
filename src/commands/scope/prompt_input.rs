use crate::config::ConfigScopeInput;
use crate::git::GitProvider;
use dialoguer::{FuzzySelect, Input};
use crate::commands::error::CommandError;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Runner {
    fn exec(&self) -> Result<ConfigScopeInput, CommandError>;
}

pub struct PromptInputRunner<'a> {
    git: &'a dyn GitProvider,
}

impl<'a> PromptInputRunner<'a> {
    pub fn new(git: &'a dyn GitProvider) -> Self {
        Self { git }
    }

    fn prompt_branch(&self, git: &dyn GitProvider) -> Result<Option<String>, CommandError> {
        let current_branch = git.get_current_branch()?;

        let branches = git.list_branches()?;

        let current_branch_option = format!("現在のブランチ: {}", current_branch);
        let mut display_branches = vec![current_branch_option];
        display_branches.extend(branches.clone());

        let branch_idx = FuzzySelect::new()
            .with_prompt("ブランチを選択してください")
            .items(&display_branches)
            .default(0)
            .interact()
            .map_err(|_| CommandError::InvalidInput("無効なブランチ選択です".to_string()))?;

        if branch_idx == 0 {
            Ok(Some(current_branch))
        } else {
            Ok(Some(branches[branch_idx - 1].clone()))
        }
    }

    fn prompt_path(&self, prompt_message: &str) -> Result<Option<String>, CommandError> {
        let path = Input::<String>::new()
            .with_prompt(prompt_message)
            .allow_empty(true)
            .interact()
            .map_err(|e| {
                CommandError::InvalidInput(format!("無効なパス入力です: {}", e))
            })?;

        if path.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    fn prompt_old_path(&self) -> Result<Option<String>, CommandError> {
        self.prompt_path("古いファイルパスを入力してください [Enter でスキップ]")
    }

    fn prompt_new_path(&self) -> Result<Option<String>, CommandError> {
        self.prompt_path("新しいファイルパスを入力してください [Enter でスキップ]")
    }
}

impl Runner for PromptInputRunner<'_> {
    fn exec(&self) -> Result<ConfigScopeInput, CommandError> {
        let branch = self.prompt_branch(self.git)?;
        let old_path = self.prompt_old_path()?;
        let new_path = self.prompt_new_path()?;

        Ok(ConfigScopeInput {
            current_branch: branch,
            old_file_path: old_path,
            new_file_path: new_path,
        })
    }
}
