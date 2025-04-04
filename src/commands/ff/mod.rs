use crate::config::Manager;

use clap::Args;
use std::io::Error;

#[derive(Args)]
pub struct FFCommand {
    pub target: String,
}

pub fn handle_ff<M: Manager>(target: String, _config: &M) -> Result<(), Error> {
    println!("FF target: {}", target);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_handle_ff() {
    //     let target = "test_target".to_string();
    //     handle_ff(target.clone());
    //     assert_eq!(target, "test_target");
    // }
}
