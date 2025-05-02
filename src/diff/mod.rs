use prettydiff::{basic::DiffOp, diff_chars, diff_lines, diff_slice, diff_words};

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DiffType {
    Added,
    Removed,
    Replaced,
    Equaled,
}

#[derive(serde::Serialize)]
pub struct LineInfo {
    pub lineno: usize,
    pub content: String,
}

#[derive(serde::Serialize)]
pub struct LineDiff {
    pub old: Option<LineInfo>,
    pub new: Option<LineInfo>,
    pub diff_type: DiffType,
}

pub trait DiffProvider {
    fn slice(&self) -> String;
    fn chars(&self) -> String;
    fn words(&self) -> String;
    fn lines(&self) -> String;
    fn lines_structured(&self) -> Vec<LineDiff>;
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

    pub fn lines_structured_with_context(&self, context: usize) -> Vec<LineDiff> {
        let old_refs: Vec<&str> = self.old.iter().map(|s| s.as_str()).collect();
        let new_refs: Vec<&str> = self.new.iter().map(|s| s.as_str()).collect();
        let changeset = diff_slice(&old_refs, &new_refs);

        let mut raw = Vec::new();
        let mut old_lineno = 1;
        let mut new_lineno = 1;

        for change in changeset.diff {
            match change {
                DiffOp::Insert(lines) => {
                    for line in lines {
                        raw.push((
                            LineDiff {
                                old: None,
                                new: Some(LineInfo {
                                    lineno: new_lineno,
                                    content: line.to_string(),
                                }),
                                diff_type: DiffType::Added,
                            },
                            true,
                        ));
                        new_lineno += 1;
                    }
                }
                DiffOp::Remove(lines) => {
                    for line in lines {
                        raw.push((
                            LineDiff {
                                old: Some(LineInfo {
                                    lineno: old_lineno,
                                    content: line.to_string(),
                                }),
                                new: None,
                                diff_type: DiffType::Removed,
                            },
                            true,
                        ));
                        old_lineno += 1;
                    }
                }
                DiffOp::Equal(lines) => {
                    for line in lines {
                        raw.push((
                            LineDiff {
                                old: Some(LineInfo {
                                    lineno: old_lineno,
                                    content: line.to_string(),
                                }),
                                new: Some(LineInfo {
                                    lineno: new_lineno,
                                    content: line.to_string(),
                                }),
                                diff_type: DiffType::Equaled,
                            },
                            false,
                        ));
                        old_lineno += 1;
                        new_lineno += 1;
                    }
                }
                DiffOp::Replace(old_lines, new_lines) => {
                    let count = old_lines.len().max(new_lines.len());

                    for i in 0..count {
                        let old = old_lines.get(i);
                        let new = new_lines.get(i);

                        let diff_type = match (old, new) {
                            (Some(_), Some(_)) => DiffType::Replaced,
                            (Some(_), None) => DiffType::Removed,
                            (None, Some(_)) => DiffType::Added,
                            (None, None) => continue,
                        };

                        raw.push((
                            LineDiff {
                                old: old.map(|line| LineInfo {
                                    lineno: old_lineno + i,
                                    content: line.to_string(),
                                }),
                                new: new.map(|line| LineInfo {
                                    lineno: new_lineno + i,
                                    content: line.to_string(),
                                }),
                                diff_type,
                            },
                            true,
                        ));
                    }

                    old_lineno += old_lines.len();
                    new_lineno += new_lines.len();
                }
            }
        }

        let mut needed = vec![false; raw.len()];
        for (i, (_, is_change)) in raw.iter().enumerate() {
            if *is_change {
                let start = i.saturating_sub(context);
                let end = (i + context + 1).min(raw.len());
                needed
                    .iter_mut()
                    .take(end)
                    .skip(start)
                    .for_each(|flag| *flag = true);
            }
        }

        raw.into_iter()
            .enumerate()
            .filter_map(|(i, (line, _))| if needed[i] { Some(line) } else { None })
            .collect()
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

    fn lines_structured(&self) -> Vec<LineDiff> {
        self.lines_structured_with_context(3)
    }
}
