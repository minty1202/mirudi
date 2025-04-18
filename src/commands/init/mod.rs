use clap::Args;
use std::io::{BufRead, Error, ErrorKind, Write, stdin, stdout};

use crate::config::Manager;

#[derive(Args)]
pub struct InitCommand {
    #[arg(long)]
    pub base: Option<String>,
}

fn prompt_for_input<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt_message: &str,
) -> Result<String, Error> {
    writeln!(writer, "{}", prompt_message)?;

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "入力が空です"));
    }

    Ok(trimmed.to_string())
}

pub fn prompt_base_branch() -> Result<String, Error> {
    let mut stdin = stdin().lock();
    let mut stdout = stdout();
    prompt_for_input(
        &mut stdin,
        &mut stdout,
        "デフォルトのブランチ名を教えてください",
    )
}

pub fn with_handle_init<F: Fn() -> Result<String, Error>>(
    cmd: InitCommand,
    config: &mut dyn Manager,
    input_fn: F,
) -> Result<(), Error> {
    let branch = match &cmd.base {
        Some(s) if s.trim().is_empty() => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "空のブランチ名は無効です",
            ));
        }
        Some(s) => s.trim().to_string(),
        None => input_fn()?,
    };

    let mut data = config
        .get_default()
        .map_err(|_| Error::new(ErrorKind::Other, "設定の取得に失敗しました"))?;
    data.set_base_branch(branch.clone())
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "ブランチ名の設定に失敗しました"))?;

    config
        .save(data)
        .map_err(|_| Error::new(ErrorKind::Other, "設定の保存に失敗しました"))?;

    println!("base_branch を '{}' に設定しました", branch);
    Ok(())
}

pub fn handle_init(cmd: InitCommand, config: &mut dyn Manager) -> Result<(), Error> {
    with_handle_init(cmd, config, prompt_base_branch)
}

#[cfg(test)]
mod tests {
    use crate::config::MockManager;
    use crate::config::data::ConfigData;
    use std::io::Error;

    use super::*;

    fn mock_prompt_for_input() -> Result<String, Error> {
        Ok("test_branch".to_string())
    }

    fn mock_handle_init(cmd: InitCommand, config: &mut dyn Manager) -> Result<(), Error> {
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
