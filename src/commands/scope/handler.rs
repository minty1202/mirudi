use crate::config::{ConfigData, ConfigScopeInput, Manager};
use crate::git::GitOperations;
use std::io::{Error, ErrorKind};

use super::core::{ScopeCommand, ScopeInputResolver};
use super::prompt_input::PromptInputRunner;
use super::prompt_input::Runner;

type ConfigManagerMut<'a> = &'a mut dyn Manager;
type MaybeGitOps<'a> = Option<&'a dyn GitOperations>;
type PromptInputFn<'a> = Box<dyn Runner + 'a>;
type BranchNameFetcher<'a> = Box<dyn Fn() -> Result<String, Error> + 'a>;

pub struct Deps<'a> {
    pub prompt_input: PromptInputFn<'a>,
    pub get_current_branch: BranchNameFetcher<'a>,
}

pub struct DepsBuilder<'a> {
    git: MaybeGitOps<'a>,
}

impl<'a> DepsBuilder<'a> {
    pub fn new() -> Self {
        Self { git: None }
    }

    pub fn git(mut self, git: &'a dyn GitOperations) -> Self {
        self.git = Some(git);
        self
    }

    pub fn build(self) -> Result<Deps<'a>, Error> {
        let git = self.git.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "git が設定されていません",
        ))?;

        let get_current_branch = Box::new(move || {
            git.get_current_branch()
                .map_err(|_| Error::new(ErrorKind::Other, "Git のブランチ名の取得に失敗しました"))
        });

        let prompt_input = Box::new(PromptInputRunner::new(git)) as Box<dyn Runner>;

        Ok(Deps {
            prompt_input,
            get_current_branch,
        })
    }
}

pub struct HandleBuilder<'a> {
    cmd: Option<ScopeCommand>,
    config: Option<ConfigManagerMut<'a>>,
    prompt_input: Option<PromptInputFn<'a>>,
    get_current_branch_name: Option<BranchNameFetcher<'a>>,
    no_display: Option<bool>,
}

impl<'a> HandleBuilder<'a> {
    pub fn new() -> Self {
        Self {
            cmd: None,
            config: None,
            prompt_input: None,
            get_current_branch_name: None,
            no_display: None,
        }
    }

    pub fn cmd(mut self, cmd: ScopeCommand) -> Self {
        self.cmd = Some(cmd);
        self
    }

    pub fn config(mut self, config: &'a mut dyn Manager) -> Self {
        self.config = Some(config);
        self
    }

    pub fn prompt_input(mut self, prompt_input: PromptInputFn<'a>) -> Self {
        self.prompt_input = Some(prompt_input);
        self
    }

    pub fn get_current_branch_name<F>(mut self, get_fn: F) -> Self
    where
        F: Fn() -> Result<String, Error> + 'a,
    {
        self.get_current_branch_name = Some(Box::new(get_fn));
        self
    }

    pub fn no_display(mut self, display: bool) -> Self {
        self.no_display = Some(display);
        self
    }

    pub fn build(self) -> Result<Handler<'a>, Error> {
        let cmd = self.cmd.ok_or(Error::new(
            std::io::ErrorKind::InvalidInput,
            "コマンドが指定されていません",
        ))?;
        let config = self.config.ok_or(Error::new(
            std::io::ErrorKind::InvalidInput,
            "設定が指定されていません",
        ))?;
        let prompt_input = self.prompt_input.ok_or(Error::new(
            std::io::ErrorKind::InvalidInput,
            "プロンプト入力が指定されていません",
        ))?;
        let get_current_branch_name = self.get_current_branch_name.ok_or(Error::new(
            std::io::ErrorKind::InvalidInput,
            "ブランチ名取得関数が指定されていません",
        ))?;
        let no_display = self.no_display.unwrap_or(false);

        Ok(Handler {
            cmd,
            config,
            prompt_input,
            get_current_branch_name,
            no_display,
        })
    }
}

pub struct Handler<'a> {
    cmd: ScopeCommand,
    config: ConfigManagerMut<'a>,
    prompt_input: PromptInputFn<'a>,
    get_current_branch_name: BranchNameFetcher<'a>,
    no_display: bool,
}

impl Handler<'_> {
    pub fn exec(&mut self) -> Result<(), Error> {
        let mut data = self.get_current_data()?;
        let input = self.get_input()?;
        data.set_scope(input);
        self.save_data(&data)?;
        let new_data = self.get_current_data()?;
        self.display_completion(&new_data);
        Ok(())
    }

    fn get_current_data(&mut self) -> Result<ConfigData, Error> {
        self.config
            .load()
            .map_err(|_| Error::new(std::io::ErrorKind::Other, "設定の読み込みに失敗しました"))
    }

    fn get_input(&self) -> Result<ConfigScopeInput, Error> {
        if self.cmd.is_empty() {
            self.prompt_input.exec()
        } else {
            Ok(ConfigScopeInput {
                current_branch: self
                    .cmd
                    .resolve_branch(self.get_current_branch_name.as_ref())?,
                old_file_path: self.cmd.resolve_old_path()?,
                new_file_path: self.cmd.resolve_new_path()?,
            })
        }
    }

    fn save_data(&mut self, data: &ConfigData) -> Result<(), Error> {
        self.config
            .save(data)
            .map_err(|_| Error::new(std::io::ErrorKind::Other, "設定の保存に失敗しました"))
    }

    fn display_completion(&self, new_data: &ConfigData) {
        if self.no_display {
            return;
        }

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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::scope::ScopeCommand;
    use crate::commands::scope::prompt_input::MockRunner;
    use crate::config::ConfigData;
    use crate::config::MockManager;
    use crate::config::error::ConfigError;
    use crate::git::core::MockGitOperations;
    use std::io::Error;

    mod deps_builder {
        use super::*;

        #[test]
        fn test_build() {
            let git = MockGitOperations::new();
            let result = DepsBuilder::new().git(&git).build();
            assert!(result.is_ok());
        }

        #[test]
        fn test_build_without_git() {
            let result = DepsBuilder::new().build();
            assert!(result.is_err());
        }
    }

    mod handle_builder {
        use super::*;

        #[test]
        fn test_build() {
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let mut config = MockManager::new();
            let prompt_input = Box::new(MockRunner::new());
            let get_current_branch_name = Box::new(|| Ok("test_branch".to_string()));

            let result = HandleBuilder::new()
                .cmd(cmd)
                .config(&mut config)
                .prompt_input(prompt_input)
                .get_current_branch_name(get_current_branch_name)
                .build();

            assert!(result.is_ok());
        }

        #[test]
        fn test_build_without_cmd() {
            let mut config = MockManager::new();
            let prompt_input = Box::new(MockRunner::new());
            let get_current_branch_name = Box::new(|| Ok("test_branch".to_string()));

            let result = HandleBuilder::new()
                .config(&mut config)
                .prompt_input(prompt_input)
                .get_current_branch_name(get_current_branch_name)
                .build();

            assert!(result.is_err());
        }

        #[test]
        fn test_build_without_config() {
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let prompt_input = Box::new(MockRunner::new());
            let get_current_branch_name = Box::new(|| Ok("test_branch".to_string()));
            let result = HandleBuilder::new()
                .cmd(cmd)
                .prompt_input(prompt_input)
                .get_current_branch_name(get_current_branch_name)
                .build();
            assert!(result.is_err());
        }

        #[test]
        fn test_build_without_prompt_input() {
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let mut config = MockManager::new();
            let get_current_branch_name = Box::new(|| Ok("test_branch".to_string()));
            let result = HandleBuilder::new()
                .cmd(cmd)
                .config(&mut config)
                .get_current_branch_name(get_current_branch_name)
                .build();
            assert!(result.is_err());
        }

        #[test]
        fn test_build_without_get_current_branch_name() {
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let mut config = MockManager::new();
            let prompt_input = Box::new(MockRunner::new());
            let result = HandleBuilder::new()
                .cmd(cmd)
                .config(&mut config)
                .prompt_input(prompt_input)
                .build();
            assert!(result.is_err());
        }
    }

    mod handler {
        use super::*;

        type TestSetupResult = (
            ScopeCommand,
            MockManager<ConfigData, ConfigError>,
            Box<dyn Runner>,
            Box<dyn Fn() -> Result<String, Error>>,
        );

        fn setup() -> TestSetupResult {
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let config = MockManager::new();
            let prompt_input = Box::new(MockRunner::new());
            let get_current_branch_name = Box::new(|| Ok("test_branch".to_string()));

            (cmd, config, prompt_input, get_current_branch_name)
        }

        #[test]
        fn test_exec() {
            let (cmd, mut config, prompt_input, get_current_branch_name) = setup();

            config
                .expect_load()
                .times(2)
                .returning(|| Ok(ConfigData::default()));

            config.expect_save().times(1).returning(|_| Ok(()));

            let mut handler = Handler {
                cmd,
                config: &mut config,
                prompt_input,
                get_current_branch_name,
                no_display: true,
            };
            let result = handler.exec();
            assert!(result.is_ok());
        }

        #[test]
        fn test_get_current_data_ok() {
            let (cmd, mut config, prompt_input, get_current_branch_name) = setup();

            config
                .expect_load()
                .times(1)
                .returning(|| Ok(ConfigData::default()));

            let mut handler = Handler {
                cmd,
                config: &mut config,
                prompt_input,
                get_current_branch_name,
                no_display: true,
            };
            let result = handler.get_current_data();
            assert!(result.is_ok());
        }

        #[test]
        fn test_get_current_data_error() {
            let (cmd, mut config, prompt_input, get_current_branch_name) = setup();
            config
                .expect_load()
                .returning(|| Err(ConfigError::EmptyBranchName));

            let mut handler = Handler {
                cmd,
                config: &mut config,
                prompt_input,
                get_current_branch_name,
                no_display: true,
            };
            let result = handler.get_current_data();
            assert!(result.is_err());
        }

        #[test]
        fn test_get_input_cmd_is_filled() {
            let (cmd, mut config, prompt_input, get_current_branch_name) = setup();

            let handler = Handler {
                cmd,
                config: &mut config,
                prompt_input,
                get_current_branch_name,
                no_display: true,
            };

            let result = handler.get_input();
            assert!(result.is_ok());
        }

        #[test]
        fn test_get_input_cmd_is_empty() {
            let (_, mut config, _, get_current_branch_name) = setup();

            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: None,
                path: None,
            };

            let mut prompt_input = MockRunner::new();
            prompt_input.expect_exec().times(1).returning(|| {
                Ok(ConfigScopeInput {
                    current_branch: Some("test_branch".to_string()),
                    old_file_path: Some("old_path".to_string()),
                    new_file_path: Some("new_path".to_string()),
                })
            });

            let boxed_prompt_input: Box<dyn Runner> = Box::new(prompt_input);

            let handler = Handler {
                cmd,
                config: &mut config,
                prompt_input: boxed_prompt_input,
                get_current_branch_name,
                no_display: true,
            };
            let result = handler.get_input();
            assert!(result.is_ok());
        }

        #[test]
        fn test_save_data_ok() {
            let (cmd, mut config, prompt_input, get_current_branch_name) = setup();

            let data = ConfigData::default();
            config.expect_save().times(1).returning(|_| Ok(()));

            let mut handler = Handler {
                cmd,
                config: &mut config,
                prompt_input,
                get_current_branch_name,
                no_display: true,
            };
            let result = handler.save_data(&data);
            assert!(result.is_ok());
        }

        #[test]
        fn test_save_data_error() {
            let (cmd, mut config, prompt_input, get_current_branch_name) = setup();

            let data = ConfigData::default();
            config
                .expect_save()
                .times(1)
                .returning(|_| Err(ConfigError::EmptyBranchName));

            let mut handler = Handler {
                cmd,
                config: &mut config,
                prompt_input,
                get_current_branch_name,
                no_display: true,
            };
            let result = handler.save_data(&data);
            assert!(result.is_err());
        }
    }
}
