#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Heading {
    pub level: u8,
    pub title: String,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub path: String,
    pub lines: Vec<String>,
    pub headings: Vec<Heading>,
}
