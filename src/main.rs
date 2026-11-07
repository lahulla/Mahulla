mod app;
mod error;
mod event;
mod model;
mod parser;
mod tui;
mod ui;

fn load_document(path: &str) -> error::Result<model::Document> {
    let content = std::fs::read_to_string(path).map_err(|source| error::AppError::Io {
        path: path.to_string(),
        source,
    })?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let headings = parser::parse_headings(&lines);
    Ok(model::Document {
        path: path.to_string(),
        lines,
        headings,
    })
}

fn run(terminal: &mut tui::Tui, app: &mut app::App) -> error::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, app))?;
        let action = event::next_action(std::time::Duration::from_millis(50))?;
        app.apply(action);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mdnav <path/to/markdown-file.md>");
        std::process::exit(1);
    }

    let path = &args[1];
    let doc = match load_document(path) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut terminal = tui::init()?;
    let mut app = app::App::new(doc);
    let run_res = run(&mut terminal, &mut app);
    tui::restore()?;
    run_res?;

    Ok(())
}
