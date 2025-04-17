use clap::Args;
use std::io::Error;

#[derive(Debug, Args, PartialEq)]
pub struct ScopeCommand {
    #[arg(short, long)]
    pub current: bool,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(short, long)]
    pub old: Option<String>,

    #[arg(short, long)]
    pub new: Option<String>,

    #[arg(short, long)]
    pub path: Option<String>,
}

pub trait ScopeInputResolver {
    fn resolve_old_path(&self) -> Result<Option<String>, Error>;
    fn resolve_new_path(&self) -> Result<Option<String>, Error>;
    fn resolve_branch<F>(&self, get_fn: F) -> Result<Option<String>, Error>
    where
        F: Fn() -> Result<String, Error>;
    fn is_empty(&self) -> bool;
}

impl ScopeCommand {
    fn resolve_path(
        &self,
        primary: &Option<String>,
        shared: &Option<String>,
    ) -> Result<Option<String>, Error> {
        if primary.is_some() && shared.is_some() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "new または old と path は併用できません",
            ));
        }

        Ok(primary.clone().or_else(|| shared.clone()))
    }
}

impl ScopeInputResolver for ScopeCommand {
    fn resolve_old_path(&self) -> Result<Option<String>, Error> {
        self.resolve_path(&self.old, &self.path)
    }

    fn resolve_new_path(&self) -> Result<Option<String>, Error> {
        self.resolve_path(&self.new, &self.path)
    }

    fn resolve_branch<F>(&self, get_fn: F) -> Result<Option<String>, Error>
    where
        F: Fn() -> Result<String, Error>,
    {
        match (&self.current, &self.branch) {
            (true, Some(_)) => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "current と branch は同時に指定できません",
            )),
            (true, None) => Ok(Some(get_fn()?)),
            (false, Some(branch)) => {
                if branch.trim().is_empty() {
                    Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "空のブランチ名は無効です",
                    ))
                } else {
                    Ok(Some(branch.clone()))
                }
            }
            (false, None) => Ok(None),
        }
    }

    fn is_empty(&self) -> bool {
        !self.current
            && self.branch.is_none()
            && self.old.is_none()
            && self.new.is_none()
            && self.path.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    mod resolve_old_path {
        use super::*;

        #[test]
        fn test_old_only_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: Some("test_old_path".to_string()),
                new: None,
                path: None,
            };

            let result = cmd.resolve_old_path();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some("test_old_path".to_string()));
        }

        #[test]
        fn test_old_and_path_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: Some("test_old_path".to_string()),
                new: None,
                path: Some("test_path".to_string()),
            };

            let result = cmd.resolve_old_path();
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
            assert_eq!(err.to_string(), "new または old と path は併用できません");
        }

        #[test]
        fn test_bath_none_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let result = cmd.resolve_old_path();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        }
    }

    mod resolve_new_path {
        use super::*;

        #[test]
        fn test_new_only_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: Some("test_new_path".to_string()),
                path: None,
            };

            let result = cmd.resolve_new_path();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some("test_new_path".to_string()));
        }

        #[test]
        fn test_new_and_path_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: Some("test_new_path".to_string()),
                path: Some("test_path".to_string()),
            };
            let result = cmd.resolve_new_path();
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
            assert_eq!(err.to_string(), "new または old と path は併用できません");
        }

        #[test]
        fn test_bath_none_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let result = cmd.resolve_new_path();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        }
    }

    mod resolve_branch {
        use super::*;

        thread_local! {
            static CALLED: RefCell<bool> = const { RefCell::new(false) };
        }

        fn mock_get_branch() -> Result<String, Error> {
            CALLED.with(|called| {
                *called.borrow_mut() = true;
            });
            Ok("test_branch".to_string())
        }

        #[test]
        fn test_current_and_branch_case() {
            let cmd = ScopeCommand {
                current: true,
                branch: Some("test_branch".to_string()),
                old: None,
                new: None,
                path: None,
            };

            let result = cmd.resolve_branch(|| Ok("test_branch".to_string()));
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
            assert_eq!(err.to_string(), "current と branch は同時に指定できません");
        }

        #[test]
        fn test_current_only_case() {
            CALLED.with(|called| *called.borrow_mut() = false);
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let result = cmd.resolve_branch(mock_get_branch);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some("test_branch".to_string()));
            CALLED.with(|called| {
                assert!(*called.borrow());
            });
        }

        #[test]
        fn test_branch_only_but_blank_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: Some("".to_string()),
                old: None,
                new: None,
                path: None,
            };
            let result = cmd.resolve_branch(|| Ok("test_branch".to_string()));
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
            assert_eq!(err.to_string(), "空のブランチ名は無効です");
        }

        #[test]
        fn test_branch_only_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: Some("test_branch".to_string()),
                old: None,
                new: None,
                path: None,
            };
            let result = cmd.resolve_branch(|| Ok("test_some_branch".to_string()));
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some("test_branch".to_string()));
        }

        #[test]
        fn test_bath_none_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            let result = cmd.resolve_branch(|| Ok("test_branch".to_string()));
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn test_all_none_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            assert!(cmd.is_empty());
        }
        #[test]
        fn test_current_only_case() {
            let cmd = ScopeCommand {
                current: true,
                branch: None,
                old: None,
                new: None,
                path: None,
            };
            assert!(!cmd.is_empty());
        }

        #[test]
        fn test_branch_only_case() {
            let cmd = ScopeCommand {
                current: false,
                branch: Some("test_branch".to_string()),
                old: None,
                new: None,
                path: None,
            };
            assert!(!cmd.is_empty());
        }
    }
}
