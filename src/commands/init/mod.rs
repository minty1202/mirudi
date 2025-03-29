pub mod base;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum InitSubcommand {
    Base { branch: String },
}

pub fn handle_init(subcommand: InitSubcommand) {
    match subcommand {
        InitSubcommand::Base { branch } => {
            base::handle_base(branch);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_init() {
        let branch = "test_branch".to_string();
        handle_init(InitSubcommand::Base {
            branch: branch.clone(),
        });
        assert_eq!(branch, "test_branch");
    }
}
