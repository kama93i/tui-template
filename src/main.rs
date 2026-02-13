use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?; // nice panic messages
    let terminal = ratatui::init(); // enters alternate screen + raw mode
    let result = run(terminal);
    ratatui::restore(); // ALWAYS restore, even on error
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Step 1: Draw
        terminal.draw(|frame| render(frame))?;

        // Step 2: Wait for input
        if let Event::Key(key) = event::read()? {
            // Step 3: React
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello! Press q to quit.", frame.area());
}
