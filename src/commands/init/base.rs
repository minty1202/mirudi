pub fn handle_base(branch: String) {
    println!("Base branch: {}", branch);
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
