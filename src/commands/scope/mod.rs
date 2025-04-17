mod core;
mod prompt_input;

pub use core::{ScopeCommand, ScopeInputResolver};
use std::io::Error;

use crate::config::{ConfigScopeInput, Manager};
use crate::git::GitOperations;

pub fn with_handle_scope<FetchFn, InputFn>(
    cmd: ScopeCommand,
    config: &mut dyn Manager,
    get_current_branch_name: FetchFn,
    input_fn: InputFn,
) -> Result<(), Error>
where
    FetchFn: Fn() -> Result<String, Error>,
    InputFn: Fn() -> Result<ConfigScopeInput, Error>,
{
    let mut data = config
        .load()
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "設定の読み込みに失敗しました"))?;

    let input = if cmd.is_empty() {
        input_fn()?
    } else {
        ConfigScopeInput {
            current_branch: cmd.resolve_branch(get_current_branch_name)?,
            old_file_path: cmd.resolve_old_path()?,
            new_file_path: cmd.resolve_new_path()?,
        }
    };

    data.set_scope(input);

    config
        .save(&data)
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "設定の保存に失敗しました"))?;

    let new_data = config
        .load()
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "設定の読み込みに失敗しました"))?;

    println!("\n設定が完了しました！");
    println!(
        "- ブランチ: {}",
        new_data.current_branch().as_deref().unwrap_or("未設定")
    );
    println!(
        "- 古いパス: {}",
        new_data.old_file_path().as_deref().unwrap_or("未設定")
    );
    println!(
        "- 新しいパス: {}",
        new_data.new_file_path().as_deref().unwrap_or("未設定")
    );

    Ok(())
}

pub fn handle_scope(
    cmd: ScopeCommand,
    config: &mut dyn Manager,
    git: &dyn GitOperations,
) -> Result<(), Error> {
    let get_current_branch_name = || {
        git.get_current_branch().map_err(|_| {
            Error::new(
                std::io::ErrorKind::Other,
                "Git のブランチ名の取得に失敗しました",
            )
        })
    };

    with_handle_scope(cmd, config, get_current_branch_name, || {
        prompt_input::run(git)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::scope::ScopeCommand;
    use crate::config::ConfigData;
    use crate::config::MockManager;
    use std::cell::RefCell;
    use std::io::Error;

    thread_local! {
        static CALLED: RefCell<bool> = const { RefCell::new(false) };
    }
    fn mock_prompt_for_input() -> Result<ConfigScopeInput, Error> {
        CALLED.with(|called| {
            *called.borrow_mut() = true;
        });
        let result = ConfigScopeInput {
            current_branch: Some("test_branch".to_string()),
            old_file_path: Some("old_path".to_string()),
            new_file_path: Some("new_path".to_string()),
        };
        Ok(result)
    }

    #[test]
    fn test_has_option_case() {
        let cmd = ScopeCommand {
            current: true,
            branch: None,
            old: Some("old_path".to_string()),
            new: Some("new_path".to_string()),
            path: None,
        };
        let mut manager = MockManager::new();
        manager
            .expect_load()
            .returning(|| Ok(ConfigData::default()));

        manager
            .expect_save()
            .withf(|data: &ConfigData| {
                assert_eq!(data.current_branch(), Some("test_branch".to_string()));
                assert_eq!(data.old_file_path(), Some("old_path".to_string()));
                assert_eq!(data.new_file_path(), Some("new_path".to_string()));
                true
            })
            .returning(|_| Ok(()));

        let result = with_handle_scope(
            cmd,
            &mut manager,
            || Ok("test_branch".to_string()),
            mock_prompt_for_input,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_option_case() {
        CALLED.with(|called| *called.borrow_mut() = false);

        let cmd = ScopeCommand {
            current: false,
            branch: None,
            old: None,
            new: None,
            path: None,
        };

        let mut manager = MockManager::new();
        manager
            .expect_load()
            .returning(|| Ok(ConfigData::default()));
        manager
            .expect_save()
            .withf(|data: &ConfigData| {
                assert_eq!(data.current_branch(), Some("test_branch".to_string()));
                assert_eq!(data.old_file_path(), Some("old_path".to_string()));
                assert_eq!(data.new_file_path(), Some("new_path".to_string()));
                true
            })
            .returning(|_| Ok(()));
        let result = with_handle_scope(
            cmd,
            &mut manager,
            || Ok("test_branch".to_string()),
            mock_prompt_for_input,
        );
        assert!(result.is_ok());
        CALLED.with(|called| {
            assert!(*called.borrow());
        });
    }
}
