use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    execute,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::io::{stdin, stdout, Write};

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
        commands: CommandList::default(),
        running: true,
        temp_content: None,
        temp_pos: None,
    };

    execute!(out, cursor::MoveTo(0, 0))?;

    write!(out, "{}", state.get_visible())?;
    out.queue(cursor::MoveTo(0, state.size.1 - 1))?;
    write!(out, "{}", state.status_bar.get_visible(&state))?;
    out.flush()?;

    enable_raw_mode()?;
    while state.running {
        let read_event = event::read()?;
        let flush = match read_event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(':') => {
                    disable_raw_mode().unwrap();
                    execute!(
                        out,
                        cursor::MoveTo(0, state.size.1 - 1),
                        Clear(ClearType::CurrentLine),
                        cursor::Show,
                        Print(":")
                    )
                    .unwrap();
                    let mut buf = String::new();
                    stdin().read_line(&mut buf).unwrap();
                    let buf = buf.lines().next().unwrap();

                    let found = state.commands.0.clone().into_iter().find(
                        |command| matches!(command, Command::Colon { cmd, .. } if *cmd == buf),
                    );
                    let retrn = if let Some(Command::Colon { func, .. }) = found {
                        func(&mut state)
                    } else {
                        false
                    };
                    write!(out, "{}", retrn).unwrap();

                    execute!(out, cursor::Hide).unwrap();
                    enable_raw_mode().unwrap();
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
