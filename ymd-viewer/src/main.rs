use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, widgets::Paragraph};
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};
#[allow(unused_imports)]
use tui_markdown::from_str;
use walkdir::WalkDir;

static MARKDOWN_DIRECTORY: &str = ".";

#[derive(Debug)]
#[allow(dead_code)]
struct AppState<'app> {
    markdown_path: &'app PathBuf,
    content: String,
    scroll_offset: u16,
}

impl<'app> AppState<'app> {
    fn new(path: &'app PathBuf) -> Self {
        let content: String =
            fs::read_to_string(path).unwrap_or_else(|_| String::from("Failed to read file."));
        Self {
            markdown_path: path,
            content,
            scroll_offset: 0,
        }
    }
}

fn find_readme(paths: &[PathBuf]) -> Option<&PathBuf> {
    paths.iter().find(|&path| path == Path::new("./README.md"))
}

fn find_first(paths: &[PathBuf]) -> Option<&PathBuf> {
    paths.first()
}

fn render_ui(f: &mut Frame, state: &AppState) {
    // Convert the raw string to stylized TUI text
    let markdown_text = tui_markdown::from_str(&state.content);

    // Create the widget
    let markdown_widget = Paragraph::new(markdown_text)
        .block(
            ratatui::widgets::Block::default()
                .title("Yet Markdown Viewer")
                .borders(ratatui::widgets::Borders::ALL),
        )
        .scroll((state.scroll_offset, 0));

    // Render it to the screen
    f.render_widget(markdown_widget, f.area());
}

fn main() -> Result<(), Box<dyn Error>> {
    let rendering_source_location = Path::new(MARKDOWN_DIRECTORY);
    #[cfg(debug_assertions)]
    {
        dbg!(rendering_source_location);
    }
    let mut markdown_library: Vec<std::path::PathBuf> = Vec::new();
    for entry in WalkDir::new(rendering_source_location)
        .into_iter()
        .filter_map(|e| e.ok()) // ignore errors
        .filter(|e| !e.file_type().is_dir()) // Filter out directories
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    // Keep only *.md files
    {
        let path = entry.path();
        println!("Adding to library: {:?}", path.display());
        markdown_library.push(path.to_path_buf());
    }
    #[cfg(debug_assertions)]
    {
        dbg!(&markdown_library);
    }
    let item: &PathBuf = find_readme(&markdown_library)
        .or_else(|| find_first(&markdown_library))
        .expect("No Markdown file found in current directory...");
    #[cfg(debug_assertions)]
    {
        dbg!(&item);
    }
    let mut state = AppState::new(item);
    #[cfg(debug_assertions)]
    {
        dbg!(&state);
    }
    enable_raw_mode().expect("Issue getting a TTY.");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Mouse handler");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|f| render_ui(f, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down | KeyCode::Char('j') => {
                    state.scroll_offset = state.scroll_offset.saturating_add(1);
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    state.scroll_offset = state.scroll_offset.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
