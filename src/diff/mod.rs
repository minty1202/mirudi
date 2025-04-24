use prettydiff::{diff_chars, diff_lines, diff_slice, diff_words};
pub trait DiffProvider {
    fn slice(&self) -> String;
    fn chars(&self) -> String;
    fn words(&self) -> String;
    fn lines(&self) -> String;
}

pub struct Diff {
    old: Vec<String>,
    new: Vec<String>,
}

impl Diff {
    pub fn new(old: Vec<String>, new: Vec<String>) -> Self {
        Self { old, new }
    }

    pub fn to_string(&self) -> (String, String) {
        let old = self.old.join("\n");
        let new = self.new.join("\n");
        (old, new)
    }
}

impl DiffProvider for Diff {
    fn slice(&self) -> String {
        let result = diff_slice(&self.old, &self.new);
        let lines: Vec<String> = format!("{result}")
            .lines()
            .filter(|line| *line != "[" && *line != "]")
            .map(|line| line.trim_end_matches(',').to_string())
            .collect();
        lines.join("\n")
    }

    fn chars(&self) -> String {
        let (old, new) = self.to_string();
        diff_chars(&old, &new)
            .set_highlight_whitespace(false)
            .format()
    }

    fn words(&self) -> String {
        let (old, new) = self.to_string();

        diff_words(&old, &new)
            .set_highlight_whitespace(false)
            .format()
    }

    fn lines(&self) -> String {
        let (old, new) = self.to_string();
        diff_lines(&old, &new).format()
    }
}
