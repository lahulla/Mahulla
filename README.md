


# Stack
- Rust 2021, stable
- ratatui = "0.29"
- ratatui-markdown = "newest"
- crossterm = "0.28"
- thiserror = "1"
- markdown-rs = "newest"

- Do not add other dependencies unless a stub below requires one.

# Directory structure

```
mdnav/
├── Cargo.toml
└── src/
    ├── main.rs        # CLI args, wiring, run loop entry
    ├── error.rs       # AppError (thiserror)
    ├── model.rs       # Heading, Document
    ├── parser.rs      # heading extraction and section slicing (pure, no I/O)
    ├── app.rs         # App state and key-driven transitions (no drawing)
    ├── event.rs       # crossterm event polling -> Action
    ├── ui.rs          # ratatui rendering only (reads &App)
    └── tui.rs         # terminal setup/teardown, panic hook
```


# Contracts

1. `parse_headings`. Read only (`#` to `###` followed by a space). It ignores any line inside a fenced code block (``` or ~~~).

2. `line_end` = `line_start` of the next heading.

3. `section_lines(doc, i)` returns `&doc.lines[line_start..line_end]` for heading `i`. It must not allocate.
4. `selected` the selected option.
5. `scroll` resets to 0 whenever `selected` changes.
6. `parser` and `app` have no I/O and no ratatui imports, so they are unit-testable.
7. `ui` reads `&App` and never mutates it.
8. `tui::restore`. Terminal is never left in raw mode.

# Relationships

- `main` → `parser` (load and parse) → `Document` → `App::new`
- `event::next_action` → `Action` → `App::apply` → mutated `App`
- `ui::draw(&App)` runs every loop iteration after `apply`
- `parser` is the only module that knows the markdown syntax rules. `ui` and `app` consume `Heading` and `Document` only.

# Keybindings

- `j` / `Down`: next heading (or scroll content when focused on content)
- `k` / `Up`: previous heading (or scroll content)
- `g` / `G`: first / last heading
- `Enter` or `l`: focus content pane
- `Esc` or `h`: Turns on `SelectionOptions`
- `PageUp` / `PageDown`: scroll content by one screen
- `q`: quit
- `SelectionOptions`: `c` (copy content), `r`(reference heading), `e`(focus on pane)

# Layout

- Left pane (~30% width): headings, indented two spaces per level minus one (`# ` at column 0, `# ` at 2, and so on). The selected row is highlighted.
- Right pane: the selected section, wrapped, scrollable.
- Bottom status line: file path, heading count, and key hints.

## Style, Color and Design

- Rounded Corners
- Dark-cyan accents and tone.
- Black-blue background (always on night mode)
- Syntax Highlight
- Modern Look and Feel.

# Deliverables and tests
- Implement every stub above with the exact signatures. Add more private helpers only if needed.
- Unit tests in `parser.rs` for: nested headings, headings inside code fences, a file with no headings, and section boundaries at EOF.
- Unit tests in `app.rs` for selection clamping and scroll reset.
- `cargo build` and `cargo test` must pass with no warnings.
- Usage: `mdnav path/to/file.md`. Exit with a clear error message if the path is missing or unreadable.

# Types and function stubs

## error.rs

```rust

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("failed to read {path}: {source}")]
    Io { path: String, #[source] source: std::io::Error },
    #[error("terminal error: {0}")]
    Terminal(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, AppError>;

```

## model.rs

Both structure implement `Debug, Clone, PartialEq` and `Eq`.
1. Individual Heading representation.
2. Entire file and it's content.

In rust this is called __traits__ in case a beginner is reaing this.

```rust

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
```

## parser.rs (pure functions)


```rust
pub fn parse_headings(lines: &[String]) -> Vec<Heading>;
pub fn section_lines<'a>(doc: &'a Document, index: usize) -> &'a [String];
```

## app.rs
```rust
pub enum Focus { Headings, Content }

pub struct App {
    pub doc: Document,
    pub selected: usize,      // index into doc.headings
    pub scroll: u16,          // content pane scroll offset
    pub focus: Focus,
    pub should_quit: bool,
}

impl App {
    pub fn new(doc: Document) -> Self;
    pub fn select_next(&mut self);
    pub fn select_prev(&mut self);
    pub fn select_first(&mut self);
    pub fn select_last(&mut self);
    pub fn scroll_by(&mut self, delta: i32);
    pub fn apply(&mut self, action: Action);
    pub fn current_content(&self) -> &[String];
}
```

### Types

- `Focus` (enum): Which pane has keyboard focus, either `Headings` (the list) or
- `Content` (the section body).
- `App` (struct) Holds the `Document`, the selected heading index, the vertical scroll offset, the current focus, and a `should_quit` flag.

#### `App` methods

##### Application, Markdown Interaction
- `new(doc)`
- `select_next()`
- `select_prev()`
- `select_first()`
- `select_last()`
- `current_content()`
- `scroll_by(delta)`
- `dispatch_action(action)`

##### Navigation 
- `Up`
- `Down`
- `First`
- `Last`

##### Focus and Selection
- `Open` focuses content
- `Back` returns focus to headings.

##### Pages and Selection
- `PageUp`
- `PageDown` scroll by 15 lines.
- `Quit` sets `should_quit`.
- `None` does nothing.

#### Tests

 `make_test_app(heading_count)`: Helper that builds an `App` with a given number of dummy level1 headings.
 `test_selection_clamping`: Checks that selection stops at the first and last heading, and that all selection methods are safe on an empty document.
 `test_scroll_reset`: Checks that changing the selection in any direction resets scroll to 0, while `scroll_by` leaves the selection unchanged.



## event.rs
```rust
pub enum Action {
    Up, Down, First, Last,
    Open,        // focus content pane
    Back,        // focus heading pane
    PageUp, PageDown,
    Quit,
    None,
}

pub fn next_action(timeout: std::time::Duration) -> std::io::Result<Action>;
```

# Design

## `ui.rs`

```rust

pub fn draw(frame: &mut ratatui::Frame, app: &App);

fn draw_headings(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App);
fn draw_content(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App);
fn draw_status(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App);

```

## `tui.rs`

```rust

pub type Tui = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;
pub fn init() -> std::io::Result<Tui>;     // raw mode, alternate screen, panic hook
pub fn restore() -> std::io::Result<()>;   // leave alternate screen, disable raw mode

```

## `main.rs`

```rust

fn main() -> Result<(), Box<dyn std::error::Error>>;
fn load_document(path: &str) -> error::Result<model::Document>;
fn run(terminal: &mut tui::Tui, app: &mut app::App) -> error::Result<()>;

```

# Syntax Highligh

In this repo,`src/ui.rs` draws the Ratatui interface, that’s where styled text should be rendered.
- `src/tui.rs` handles terminal setup.

# Expected Behavior

1. `Tui`instance that writes to standard
2. `init()` into raw mode
(disables line buffering and input echo).

Switches to the alternate screen buffer and hides the mouse cursor.
Sets a panic hook to run `restore()` before exiting if the program crashes, preventing terminal corruption.

```
(init) -> ratatui::Terminal.
```
 `restore()` : Reverses the setup—leaves the alternate screen, unhides the cursor, and disables raw mode.

Syntax highlighting  requires parsing and styling libraries. Some options are...

- `syntect`
- `tree-sitter`
- `tui-syntax-highlight`

Apply colors

- `Style`
- `Color`
