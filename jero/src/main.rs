mod about;
use crate::about::About;
use std::{
    error::Error,
    io::self
};
use crossterm::{
    execute,
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        SetTitle
    }
};
use ratatui::{backend::CrosstermBackend, Terminal};
// ---------------------------------------------------------------------------------------------- //
pub fn main() -> io::Result<()>
{
    let mut stdout = io::stdout();

    let _ = update_terminal_title(&mut stdout);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let about_widget = About::new(None);
    loop {
        terminal.draw(|frame| 
            frame.render_widget(&mut about_widget.clone(), frame.area()))?;
        // Handle events (e.g., exit on 'q' press)
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

fn update_terminal_title(stdout: &mut std::io::Stdout) 
    -> Result<(), Box<dyn Error + 'static>> 
{
    let pkg = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let bin = env!("CARGO_BIN_NAME");
    terminal::enable_raw_mode()?;
    let _ = execute!(
        stdout,
        EnterAlternateScreen,
        SetTitle(format!("{} v{} :: {}",pkg,pkg_version,bin))
    )?;
    Ok(())
}
