use std::io;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::Frame;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

use termion::raw::IntoRawMode;

fn main() -> Result<(), io::Error> {
    // create stdout for into raw mode.
    let stdout = io::stdout().into_raw_mode()?;
    // create termion backend.
    let backend = TermionBackend::new(stdout);
    // create window.
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f: Frame<_>| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(f.size().width / 2),
                    Constraint::Percentage(1),
                    Constraint::Percentage(f.size().width)
                ].as_ref()
            )
            .split(f.size());
        // create block.
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, chunks[0]);
        Block::default()
            .title("Block 2")
            .borders(Borders::ALL)
            .render(&mut f, chunks[2]);
    })
}