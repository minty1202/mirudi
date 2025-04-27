use crate::config::ConfigScopeInput;
use crate::git::GitProvider;
use dialoguer::{FuzzySelect, Input};
use std::io::Error;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Runner {
    fn exec(&self) -> Result<ConfigScopeInput, Error>;
}

pub struct PromptInputRunner<'a> {
    git: &'a dyn GitProvider,
}

impl<'a> PromptInputRunner<'a> {
    pub fn new(git: &'a dyn GitProvider) -> Self {
        Self { git }
    }

    fn prompt_branch(&self, git: &dyn GitProvider) -> Result<Option<String>, Error> {
        let current_branch = git.get_current_branch().map_err(|_| {
            Error::new(
                std::io::ErrorKind::Other,
                "Git のブランチ名の取得に失敗しました",
            )
        })?;

        let branches = git.list_branches().map_err(|_| {
            Error::new(
                std::io::ErrorKind::Other,
                "Git のブランチ一覧の取得に失敗しました",
            )
        })?;

        let current_branch_option = format!("現在のブランチ: {}", current_branch);
        let mut display_branches = vec![current_branch_option];
        display_branches.extend(branches.clone());

        let branch_idx = FuzzySelect::new()
            .with_prompt("ブランチを選択してください")
            .items(&display_branches)
            .default(0)
            .interact()
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "無効なブランチ選択です"))?;

        if branch_idx == 0 {
            Ok(Some(current_branch))
        } else {
            Ok(Some(branches[branch_idx - 1].clone()))
        }
    }

    fn prompt_path(&self, prompt_message: &str) -> Result<Option<String>, Error> {
        let path = Input::<String>::new()
            .with_prompt(prompt_message)
            .allow_empty(true)
            .interact()
            .map_err(|e| {
                Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("無効なパス入力です: {}", e),
                )
            })?;

        if path.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    fn prompt_old_path(&self) -> Result<Option<String>, Error> {
        self.prompt_path("古いファイルパスを入力してください [Enter でスキップ]")
    }

    fn prompt_new_path(&self) -> Result<Option<String>, Error> {
        self.prompt_path("新しいファイルパスを入力してください [Enter でスキップ]")
    }
}

impl Runner for PromptInputRunner<'_> {
    fn exec(&self) -> Result<ConfigScopeInput, Error> {
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
