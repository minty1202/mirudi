use crate::config::CONFIG;
use clap::Args;

#[derive(Args)]
pub struct InitCommand {
    #[arg(long)]
    pub base: Option<String>,
}

pub fn handle_init(cmd: InitCommand) {
    let branch = match &cmd.base {
        Some(s) if s.trim().is_empty() => {
            #[cfg(test)]
            panic!("空のブランチ名は無効です");

            #[cfg(not(test))]
            {
                eprintln!("空のブランチ名は無効です。");
                std::process::exit(1);
            }
        }
        Some(s) => s.trim().to_string(),
        None => prompt_base_branch()
    };

    // TODO: ここでブランチ名のバリデーションを行う
    // 例: 正規表現を使ってブランチ名が有効かどうかをチェックなど
    if let Ok(mut config) = CONFIG.write() {
        config.save_base_branch(branch.clone());
        println!("base_branch を '{}' に設定しました", branch);
    } else {
        eprintln!("Config のロックに失敗しました！");
    }
}

fn prompt_base_branch() -> String {
    use std::io::{stdin, stdout, Write};

    println!("デフォルトのブランチ名を教えてください");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_init() {
        let branch = "test_branch".to_string();
        handle_init(InitCommand {
            base: Some(branch.clone()),
        });
        assert_eq!(branch, "test_branch");

        let config = CONFIG.read().unwrap();
        assert_eq!(config.base_branch(), Some(branch.clone()));

        let result = std::panic::catch_unwind(|| {
            handle_init(InitCommand {
                base: Some("".to_string()), // 空文字
            });
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            handle_init(InitCommand {
                base: None,
            });
        });
        assert!(result.is_ok());
    }
}
