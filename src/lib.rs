use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdin, stdout, Write};

mod status_bar;
pub use status_bar::*;
mod state;
pub use state::*;

pub fn run(state: &mut State) -> std::io::Result<()> {
    let mut out = stdout();
    disable_raw_mode()?;
    execute!(
        out,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(state.get_visible()),
        cursor::MoveTo(0, state.size.1 - state.status_bar.line_layouts.len() as u16),
        Print(state.status_bar.get_visible(state))
    )?;
    enable_raw_mode()?;

    while state.running {
        let read_event = event::read()?;
        let flush = match read_event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(':') => {
                    disable_raw_mode()?;
                    execute!(
                        out,
                        cursor::MoveTo(0, state.size.1 - 1),
                        Clear(ClearType::CurrentLine),
                        cursor::Show,
                        Print(":")
                    )?;
                    let mut buf = String::new();
                    stdin().read_line(&mut buf)?;
                    let buf = buf.lines().next().unwrap();

                    let found = state.commands.0.clone().into_iter().find(
                        |command| matches!(command, Command { cmd, .. } if cmd.contains(&CommandType::Colon(buf.to_string()))),
                    );
                    let retrn = if let Some(Command { func, .. }) = found {
                        func(state)
                    } else {
                        false
                    };

                    execute!(out, Print(retrn), cursor::Hide)?;
                    enable_raw_mode()?;
                    retrn
                }
                code => state.match_key_event(code),
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
        if flush {
            disable_raw_mode()?;
            queue!(
                out,
                cursor::MoveTo(0, 0),
                terminal::Clear(ClearType::All),
                Print(state.get_visible()),
                cursor::MoveTo(0, state.size.1 - state.status_bar.line_layouts.len() as u16),
                Print(state.status_bar.get_visible(state)),
            )?;
            out.flush()?;
            enable_raw_mode()?;
        }
    }

    disable_raw_mode()?;

    Ok(())
}

pub fn init() -> std::io::Result<()> {
    let mut out = stdout();
    execute!(
        out,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture,
        cursor::Hide,
    )
}

pub fn finish() -> std::io::Result<()> {
    let mut out = stdout();
    execute!(
        out,
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen,
        cursor::Show
    )
}
