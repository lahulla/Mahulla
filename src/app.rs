
use crate::event::Action;
use crate::model::Document;
use crate::parser::section_lines;

pub enum Focus {
    Headings,
    Content,
}

pub struct App {
    pub doc: Document,
    pub selected: usize,
    pub scroll: u16,
    pub focus: Focus,
    pub should_quit: bool,
}

impl App {
    pub fn new(doc: Document) -> Self {
        Self {
            doc,
            selected: 0,
            scroll: 0,
            focus: Focus::Headings,
            should_quit: false,
        }
    }

    pub fn select_next(&mut self) {
        if self.doc.headings.is_empty() {
            return;
        }
        if self.selected + 1 < self.doc.headings.len() {
            self.selected += 1;
            self.scroll = 0;
        }
    }

    pub fn select_prev(&mut self) {
        if self.doc.headings.is_empty() {
            return;
        }
        if self.selected > 0 {
            self.selected -= 1;
            self.scroll = 0;
        }
    }

    pub fn select_first(&mut self) {
        if self.doc.headings.is_empty() {
            return;
        }
        if self.selected != 0 {
            self.selected = 0;
            self.scroll = 0;
        }
    }

    pub fn select_last(&mut self) {
        if self.doc.headings.is_empty() {
            return;
        }
        let last = self.doc.headings.len() - 1;
        if self.selected != last {
            self.selected = last;
            self.scroll = 0;
        }
    }

    pub fn scroll_by(&mut self, delta: i32) {
        if delta < 0 {
            self.scroll = self.scroll.saturating_sub((-delta) as u16);
        } else {
            self.scroll = self.scroll.saturating_add(delta as u16);
        }
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Up => match self.focus {
                Focus::Headings => self.select_prev(),
                Focus::Content => self.scroll_by(-1),
            },
            Action::Down => match self.focus {
                Focus::Headings => self.select_next(),
                Focus::Content => self.scroll_by(1),
            },
            Action::First => self.select_first(),
            Action::Last => self.select_last(),
            Action::Open => self.focus = Focus::Content,
            Action::Back => self.focus = Focus::Headings,
            Action::PageUp => self.scroll_by(-15),
            Action::PageDown => self.scroll_by(15),
            Action::Quit => self.should_quit = true,
            Action::None => {}
        }
    }

    pub fn current_content(&self) -> &[String] {
        section_lines(&self.doc, self.selected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Heading;

    fn make_test_app(heading_count: usize) -> App {
        let lines = vec!["line".to_string()];
        let headings = (0..heading_count)
            .map(|i| Heading {
                level: 1,
                title: format!("Heading {}", i),
                line_start: 0,
                line_end: 1,
            })
            .collect();
        let doc = Document {
            path: "test.md".into(),
            lines,
            headings,
        };
        App::new(doc)
    }

    #[test]
    fn test_selection_clamping() {
        let mut app = make_test_app(3);
        assert_eq!(app.selected, 0);

        app.select_prev();
        assert_eq!(app.selected, 0);

        app.select_next();
        assert_eq!(app.selected, 1);
        app.select_next();
        assert_eq!(app.selected, 2);
        app.select_next();
        assert_eq!(app.selected, 2);

        app.select_first();
        assert_eq!(app.selected, 0);
        app.select_last();
        assert_eq!(app.selected, 2);

        let mut empty_app = make_test_app(0);
        assert_eq!(empty_app.selected, 0);
        empty_app.select_next();
        assert_eq!(empty_app.selected, 0);
        empty_app.select_prev();
        assert_eq!(empty_app.selected, 0);
        empty_app.select_first();
        assert_eq!(empty_app.selected, 0);
        empty_app.select_last();
        assert_eq!(empty_app.selected, 0);
    }

    #[test]
    fn test_scroll_reset() {
        let mut app = make_test_app(3);
        app.scroll_by(10);
        assert_eq!(app.scroll, 10);

        app.select_next();
        assert_eq!(app.selected, 1);
        assert_eq!(app.scroll, 0);

        app.scroll_by(5);
        assert_eq!(app.scroll, 5);

        app.select_prev();
        assert_eq!(app.selected, 0);
        assert_eq!(app.scroll, 0);

        app.scroll_by(8);
        assert_eq!(app.scroll, 8);
        app.select_last();
        assert_eq!(app.selected, 2);
        assert_eq!(app.scroll, 0);

        app.scroll_by(12);
        assert_eq!(app.scroll, 12);
        app.select_first();
        assert_eq!(app.selected, 0);
        assert_eq!(app.scroll, 0);
    }
}
