use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Write};

mod status_bar;
pub use status_bar::*;
mod state;
pub use state::*;

pub fn run(content: String, status_bar: StatusBar) -> std::io::Result<()> {
    let mut out = stdout();
    out.queue(cursor::Hide)?;
    out.queue(terminal::EnterAlternateScreen)?;
    execute!(out, event::EnableMouseCapture)?;

    let mut state = State {
        pos: (0, 0),
        size: terminal::size()?,
        content,
        status_bar,
    };

    execute!(out, cursor::MoveTo(0, 0))?;

    write!(out, "{}", state.get_visible())?;
    out.queue(cursor::MoveTo(0, state.size.1 - 1))?;
    write!(out, "{}", state.status_bar.get_visible(&state))?;
    out.flush()?;

    enable_raw_mode()?;
    loop {
        let read_event = event::read()?;
        let flash = match read_event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Up => state.up(),
                KeyCode::Down => state.down(),
                KeyCode::Left => state.left(),
                KeyCode::Right => state.right(),
                KeyCode::Home => state.home(),
                KeyCode::End => state.end(),
                KeyCode::PageUp => state.pgup(),
                KeyCode::PageDown => state.pgdown(),
                KeyCode::Char('Q') | KeyCode::Char('q') | KeyCode::Esc => break,
                _ => false,
            },
            Event::Mouse(ev) => match ev {
                MouseEvent {
                    kind: MouseEventKind::ScrollUp,
                    ..
                } => state.up(),
                MouseEvent {
                    kind: MouseEventKind::ScrollDown,
                    ..
                } => state.down(),
                _ => false,
            },
            Event::Resize(x, y) => {
                state.size = (x, y);
                true
            }
        };
        if flash {
            disable_raw_mode()?;
            execute!(out, cursor::MoveTo(0, 0))?;
            out.queue(terminal::Clear(ClearType::All))?;
            write!(out, "{}", state.get_visible())?;
            out.queue(cursor::MoveTo(
                0,
                state.size.1 - state.status_bar.line_layouts.len() as u16,
            ))?;
            write!(out, "{}", state.status_bar.get_visible(&state))?;
            out.flush()?;
            enable_raw_mode()?;
        }
    }

    disable_raw_mode()?;
    execute!(out, event::DisableMouseCapture)?;
    out.queue(cursor::Show)?;
    out.queue(terminal::LeaveAlternateScreen)?;

    Ok(())
}
