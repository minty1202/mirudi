use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    let is_release_binary = std::env::current_exe()
        .map(|path| !path.to_string_lossy().contains("target/debug"))
        .unwrap_or(true);

    if is_release_binary {
        dirs::home_dir().unwrap()
    } else {
        std::env::current_dir().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirudi_config_dir() {
        let expected = std::env::current_dir().unwrap();
        let actual = config_dir();

        assert_eq!(actual, expected);
    }
}
