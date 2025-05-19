use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Tabs, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::Spans,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) });

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let tabs = vec!["Home", "Stats", "About"];
    let mut current_tab = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                ])
                .split(f.size());

            let tab_titles: Vec<Spans> = tabs
                .iter()
                .map(|t| Spans::from(*t))
                .collect();

            let tabs_widget = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(current_tab)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow));

            f.render_widget(tabs_widget, chunks[0]);

            let content = match current_tab {
                0 => "Welcome to the Home tab.",
                1 => "📊 Stats: Everything is running great!",
                2 => "🦀 About: This app is built with Rust + Ratatui.",
                _ => "",
            };

            let paragraph = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL).title("Content"));

            f.render_widget(paragraph, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => current_tab = (current_tab + 1) % 3,
                    KeyCode::Left => current_tab = (current_tab + 2) % 3, // wrap around
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
