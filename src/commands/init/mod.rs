// use crate::config::CONFIG;

use clap::Args;
use std::io::{stdin, stdout, BufRead, Write, Error, ErrorKind};

use crate::config::Manager;

#[derive(Args)]
pub struct InitCommand {
    #[arg(long)]
    pub base: Option<String>,
}

pub fn handle_init<M: Manager>(cmd: InitCommand, config: &M) -> Result<(), Error> {
    let branch = match &cmd.base {
        Some(s) if s.trim().is_empty() => {
            return Err(Error::new(ErrorKind::InvalidInput, "空のブランチ名は無効です"));
        }
        Some(s) => s.trim().to_string(),
        None => prompt_base_branch()?  // ? 演算子でエラーを伝播
    };

    // TODO: ここでブランチ名のバリデーションを行う
    // 例: 正規表現を使ってブランチ名が有効かどうかをチェックなど
    // let mut config = CONFIG.write()
    //     .map_err(|_| Error::new(ErrorKind::Other, "Config のロックに失敗しました"))?;

    // config.save_base_branch(branch.clone());
    // println!("base_branch を '{}' に設定しました", branch);
    Ok(())
}

fn prompt_for_input<R: BufRead, W: Write>(
    reader: &mut R, 
    writer: &mut W,
    prompt_message: &str
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
    prompt_for_input(&mut stdin, &mut stdout, "デフォルトのブランチ名を教えてください")
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_handle_init() {
    //     let branch = "test_branch".to_string();
    //     handle_init(InitCommand {
    //         base: Some(branch.clone()),
    //     });
    //     assert_eq!(branch, "test_branch");

    //     // let config = CONFIG.read().unwrap();
    //     // assert_eq!(config.base_branch(), Some(branch.clone()));

    //     let result = std::panic::catch_unwind(|| {
    //         handle_init(InitCommand {
    //             base: Some("".to_string()), // 空文字
    //         });
    //     });
    //     assert!(result.is_err());

    //     let result = std::panic::catch_unwind(|| {
    //         handle_init(InitCommand {
    //             base: None,
    //         });
    //     });
    //     assert!(result.is_ok());
    // }
}
