use clap::Args;
use std::io::{BufRead, Write, stdin, stdout};

use crate::config::Manager;
use crate::commands::error::CommandError;

#[derive(Args)]
pub struct InitCommand {
    #[arg(long)]
    pub base: Option<String>,
}

fn prompt_for_input<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt_message: &str,
) -> Result<String, CommandError> {
    writeln!(writer, "{}", prompt_message)?;

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CommandError::InvalidInput("入力が空です".to_string()));
    }

    Ok(trimmed.to_string())
}

pub fn prompt_base_branch() -> Result<String, CommandError> {
    let mut stdin = stdin().lock();
    let mut stdout = stdout();
    prompt_for_input(
        &mut stdin,
        &mut stdout,
        "デフォルトのブランチ名を教えてください",
    )
}

fn with_handle_init<F: Fn() -> Result<String, CommandError>>(
    cmd: InitCommand,
    config: &mut dyn Manager,
    input_fn: F,
) -> Result<(), CommandError> {
    let branch = match &cmd.base {
        Some(s) if s.trim().is_empty() => {
            return Err(CommandError::IO("空のブランチ名は無効です".to_string()));
        }
        Some(s) => s.trim().to_string(),
        None => input_fn()?,
    };

    let mut data = config
        .get_default()?;
    data.set_base_branch(branch.clone())?;

    config
        .save(&data)?;

    println!("base_branch を '{}' に設定しました", branch);
    Ok(())
}

pub fn handle(cmd: InitCommand, config: &mut dyn Manager) -> Result<(), CommandError> {
    with_handle_init(cmd, config, prompt_base_branch)
}

#[cfg(test)]
mod tests {
    use crate::config::ConfigData;
    use crate::config::MockManager;

    use super::*;

    fn mock_prompt_for_input() -> Result<String, CommandError> {
        Ok("test_branch".to_string())
    }

    fn mock_handle_init(cmd: InitCommand, config: &mut dyn Manager) -> Result<(), CommandError> {
        with_handle_init(cmd, config, mock_prompt_for_input)
    }

    #[test]
    fn test_handle_init_with_base_option() {
        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_get_default()
            .returning(|| Ok(ConfigData::default()));
        mock_manager
            .expect_save()
            .withf(|data| data.base_branch() == Some("test_branch".to_string()))
            .returning(|_| Ok(()));
        let cmd = InitCommand {
            base: Some("test_branch".to_string()),
        };
        let result = mock_handle_init(cmd, &mut mock_manager);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_init_with_empty_base_option() {
        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_get_default()
            .returning(|| Ok(ConfigData::default()));
        mock_manager.expect_save().returning(|_| Ok(()));
        let cmd = InitCommand {
            base: Some("".to_string()),
        };
        let result = mock_handle_init(cmd, &mut mock_manager);

        assert!(result.is_err());
    }

    #[test]
    fn test_handle_init_with_none_base_option() {
        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_get_default()
            .returning(|| Ok(ConfigData::default()));
        mock_manager
            .expect_save()
            .withf(|data| data.base_branch() == Some("test_branch".to_string()))
            .returning(|_| Ok(()));
        let cmd = InitCommand { base: None };
        let result = mock_handle_init(cmd, &mut mock_manager);

        assert!(result.is_ok());
    }
}
