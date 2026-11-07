use crate::model::{Document, Heading};

fn fence_marker(line: &str) -> Option<(char, usize)> {
    let trimmed = line.trim_start();
    let mut chars = trimmed.chars();
    let first = chars.next()?;
    if first == '`' {
        let count = 1 + chars.take_while(|&c| c == first).count();
        if count >= 3 {
            return Some((first, count));
        }
    }
    None
}

fn is_closing_fence(line: &str, active_char: char, active_len: usize) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with(active_char) {
        let count = trimmed.chars().take_while(|&c| c == active_char).count();
        if count >= active_len && trimmed[count..].trim().is_empty() {
            return true;
        }
    }
    false
}

fn parse_atx_heading(line: &str) -> Option<(u8, String)> {
    let trimmed = line.trim_start();
    let leading_spaces = line.len() - trimmed.len();

    if leading_spaces > 3 {
        return None;
    }

    let bytes = trimmed.as_bytes();
    let mut level = 0;
    while level < bytes.len() && bytes[level] == b'#' {
        level += 1;
    }

    if (1..=6).contains(&level) && bytes.get(level) == Some(&b' ') {
        let title = trimmed[level + 1..].trim().to_string();
        Some((level as u8, title))
    } else {
        None
    }
}

pub fn parse_headings(lines: &[String]) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut in_fence: Option<(char, usize)> = None;

    for (idx, line) in lines.iter().enumerate() {
        if let Some((f_char, f_len)) = in_fence {
            if is_closing_fence(line, f_char, f_len) {
                in_fence = None;
            }
            continue;
        }

        if let Some((f_char, f_len)) = fence_marker(line) {
            in_fence = Some((f_char, f_len));
            continue;
        }

        if let Some((level, title)) = parse_atx_heading(line) {
            headings.push(Heading {
                level,
                title,
                line_start: idx,
                line_end: lines.len(),
            });
        }
    }

    let mut stack: Vec<usize> = Vec::new();
    for i in 0..headings.len() {
        while let Some(&top) = stack.last() {
            if headings[i].level <= headings[top].level {
                headings[top].line_end = headings[i].line_start;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    headings
}

pub fn section_lines<'a>(doc: &'a Document, index: usize) -> &'a [String] {
    match doc.headings.get(index) {
        Some(h) => {
            let start = h.line_start.min(doc.lines.len());
            let end = h.line_end.min(doc.lines.len());
            if start <= end {
                &doc.lines[start..end]
            } else {
                &[]
            }
        }
        None => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_headings() {
        let lines: Vec<String> = vec![
            "# Top".into(),
            "intro".into(),
            "## Child 1".into(),
            "child 1 body".into(),
            "### Grandchild".into(),
            "grandchild body".into(),
            "## Child 2".into(),
            "child 2 body".into(),
        ];
        let headings = parse_headings(&lines);
        assert_eq!(headings.len(), 4);

        assert_eq!(headings[0].title, "Top");
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].line_start, 0);
        assert_eq!(headings[0].line_end, 8);

        assert_eq!(headings[1].title, "Child 1");
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[1].line_start, 2);
        assert_eq!(headings[1].line_end, 6);

        assert_eq!(headings[2].title, "Grandchild");
        assert_eq!(headings[2].level, 3);
        assert_eq!(headings[2].line_start, 4);
        assert_eq!(headings[2].line_end, 6);

        assert_eq!(headings[3].title, "Child 2");
        assert_eq!(headings[3].level, 2);
        assert_eq!(headings[3].line_start, 6);
        assert_eq!(headings[3].line_end, 8);
    }

    #[test]
    fn test_headings_inside_code_fences() {
        let lines: Vec<String> = vec![
            "# Real Heading".into(),
            "```markdown".into(),
            "# Fake Heading".into(),
            "## Another Fake".into(),
            "```".into(),
            "~~~".into(),
            "### Fake in Tildes".into(),
            "~~~".into(),
            "## Real Subheading".into(),
        ];
        let headings = parse_headings(&lines);
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].title, "Real Heading");
        assert_eq!(headings[0].line_start, 0);
        assert_eq!(headings[1].title, "Real Subheading");
        assert_eq!(headings[1].line_start, 8);
    }

    #[test]
    fn test_file_with_no_headings() {
        let lines: Vec<String> = vec![
            "Just a line".into(),
            "Another line without headings".into(),
            "```rust".into(),
            "fn main() {}".into(),
            "```".into(),
        ];
        let headings = parse_headings(&lines);
        assert!(headings.is_empty());

        let doc = Document {
            path: "test.md".into(),
            lines,
            headings,
        };
        assert_eq!(section_lines(&doc, 0), &[] as &[String]);
    }

    #[test]
    fn test_section_boundaries_at_eof() {
        let lines: Vec<String> = vec![
            "# Heading 1".into(),
            "text 1".into(),
            "# Heading 2".into(),
            "text 2".into(),
            "trailing text".into(),
        ];
        let headings = parse_headings(&lines);
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].line_start, 0);
        assert_eq!(headings[0].line_end, 2);
        assert_eq!(headings[1].line_start, 2);
        assert_eq!(headings[1].line_end, 5);

        let doc = Document {
            path: "test.md".into(),
            lines: lines.clone(),
            headings,
        };
        assert_eq!(section_lines(&doc, 0), &lines[0..2]);
        assert_eq!(section_lines(&doc, 1), &lines[2..5]);
    }
}
