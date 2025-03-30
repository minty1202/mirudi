use crate::config::CONFIG;

pub fn handle_base(branch: String) {
    let mut config = CONFIG.write().expect("Config のロックに失敗しました！");
    config.save_base_branch(branch.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_base() {
        let branch = "test_branch".to_string();
        handle_base(branch.clone());
        assert_eq!(branch, "test_branch");
    }
}
