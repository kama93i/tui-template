// =============================================================================
// Ratatui TUI Template — Clone this and customize for each new app
//
// Architecture:
//   1. main()        — init terminal, run loop, restore terminal
//   2. App struct    — all your application state lives here
//   3. run()         — the core loop: draw → read input → update state
//   4. render()      — builds the UI from current state (immediate mode)
//   5. handle_input() — maps keypresses to state changes
//   6. execute_command() — YOUR CUSTOM LOGIC GOES HERE
//
// To make a new app from this template:
//   1. Copy the project, rename in Cargo.toml
//   2. Add fields to App for your state
//   3. Add your commands in execute_command()
//   4. Customize render() if you need more panels/widgets
// =============================================================================

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

// =============================================================================
// App State — add whatever your app needs here
// =============================================================================
struct App {
    running: bool,
    input: String,         // what the user is currently typing
    messages: Vec<String>, // output history / log
}

impl App {
    fn new() -> Self {
        Self {
            running: true,
            input: String::new(),
            messages: vec![
                "Welcome! Type 'help' for available commands.".into(),
                "Press Esc to quit.".into(),
            ],
        }
    }
}

// =============================================================================
// Entry point
// =============================================================================
fn main() -> Result<()> {
    // Install color-eyre for nice panic/error backtraces
    color_eyre::install()?;

    // ratatui::init() does three things:
    //   1. Switches to the alternate screen (so your app doesn't trash scroll history)
    //   2. Enables raw mode (keypresses arrive immediately, no line buffering)
    //   3. Returns a Terminal handle you draw to
    let terminal = ratatui::init();

    let result = run(terminal);

    // ALWAYS restore the terminal, even if the app errored.
    // This undoes raw mode + alternate screen so the user's shell is normal again.
    ratatui::restore();

    result
}

// =============================================================================
// Core loop: draw → read → update
// =============================================================================
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    while app.running {
        // Draw the entire UI based on current state
        terminal.draw(|frame| render(frame, &app))?;

        // Block until the user does something (key press, mouse, resize)
        if let Event::Key(key) = event::read()? {
            handle_input(&mut app, key);
        }
    }

    Ok(())
}

// =============================================================================
// Rendering — builds the UI each frame from app state
//
// Layout:
// ┌─────────────── Output ───────────────┐
// │ Welcome! Type 'help' for commands.   │
// │ > hello                              │
// │   Hello, world!                      │
// │                                      │
// └──────────────────────────────────────┘
// ┌─────────────── Command ──────────────┐
// │ your typing here█                    │
// └──────────────────────────────────────┘
// =============================================================================
fn render(frame: &mut Frame, app: &App) {
    // Split the terminal vertically: big top area + small input bar at bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // output area — takes all remaining space
            Constraint::Length(3), // input bar — exactly 3 rows (1 text + 2 border)
        ])
        .split(frame.area());

    // --- Output panel (chunks[0]) ---
    let messages_text = app.messages.join("\n");

    // If messages exceed visible area, auto-scroll to bottom
    let visible_height = chunks[0].height.saturating_sub(2) as usize; // -2 for borders
    let total_lines = app.messages.len();
    let scroll_offset = total_lines.saturating_sub(visible_height) as u16;

    let output = Paragraph::new(messages_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Output ")
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .scroll((scroll_offset, 0)); // (vertical_scroll, horizontal_scroll)

    frame.render_widget(output, chunks[0]);

    // --- Input bar (chunks[1]) ---
    let input_bar = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Command ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(input_bar, chunks[1]);

    // Place the blinking cursor after the typed text inside the input bar
    // +1 on each axis to account for the border
    frame.set_cursor_position((chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1));
}

// =============================================================================
// Input handling — maps key events to state changes
// =============================================================================
fn handle_input(app: &mut App, key: KeyEvent) {
    // On Windows, crossterm sends both Press and Release events.
    // Only handle Press to avoid double-firing.
    if key.kind != KeyEventKind::Press {
        return;
    }

    match key.code {
        // User pressed Enter — submit the command
        KeyCode::Enter => {
            let command: String = app.input.drain(..).collect();
            if !command.is_empty() {
                // Echo the command to the output
                app.messages.push(format!("> {}", command));
                execute_command(app, &command);
            }
        }

        // Typing a character — append to input
        KeyCode::Char(c) => {
            app.input.push(c);
        }

        // Backspace — delete last character
        KeyCode::Backspace => {
            app.input.pop();
        }

        // Esc — quit the app
        KeyCode::Esc => {
            app.running = false;
        }

        // Anything else — ignore
        _ => {}
    }
}

// =============================================================================
// Command execution — THIS IS WHAT YOU CUSTOMIZE PER APP
//
// When you clone this template for a new project, this is the main function
// you'll rewrite. Add your own commands, call into your own modules, etc.
// =============================================================================
fn execute_command(app: &mut App, cmd: &str) {
    match cmd.trim() {
        "help" => {
            app.messages.push("  Available commands:".into());
            app.messages.push("    help   — show this message".into());
            app.messages.push("    hello  — say hello".into());
            app.messages.push("    clear  — clear the output".into());
            app.messages.push("    quit   — exit the app".into());
        }
        "hello" => {
            app.messages.push("  Hello, world!".into());
        }
        "clear" => {
            app.messages.clear();
        }
        "quit" => {
            app.running = false;
        }
        other => {
            app.messages
                .push(format!("  Unknown command: '{}'. Try 'help'.", other));
        }
    }
}
