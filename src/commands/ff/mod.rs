pub fn handle_ff(target: String) {
    println!("FF target: {}", target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_ff() {
        let target = "test_target".to_string();
        handle_ff(target.clone());
        assert_eq!(target, "test_target");
    }
}
