use crate::commands::error::CommandError;

pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn parse(range: &str) -> Result<Self, CommandError> {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err(CommandError::ArgParse(format!(
                "範囲 '{}' が正しくありません。例: 1-10",
                range
            )));
        }

        let start = parts[0].parse::<usize>().map_err(|_| {
            CommandError::ArgParse(format!("開始値 '{}' が数値ではありません。", parts[0]))
        })?;

        let end = parts[1].parse::<usize>().map_err(|_| {
            CommandError::ArgParse(format!("終了値 '{}' が数値ではありません。", parts[1]))
        })?;

        if start > end {
            return Err(CommandError::InvalidInput(format!(
                "開始値 '{}' は終了値 '{}' より大きくはできません。",
                start, end
            )));
        }

        Ok(Range { start, end })
    }

    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_range() {
        let range = Range::parse("1-10").unwrap();
        assert_eq!(range.start(), 1);
        assert_eq!(range.end(), 10);
    }

    #[test]
    fn test_parse_invalid_range() {
        let result = Range::parse("10-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_non_numeric_range() {
        let result = Range::parse("a-b");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_range_format() {
        let result = Range::parse("1-1a");
        assert!(result.is_err());
    }
}
