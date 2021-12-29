use std::{
    env,
    fs::File,
    io::{stdout, Read, Write},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    QueueableCommand,
};
fn line_indicator_format(line_num: usize, line_count: usize) -> String {
    let str_num = line_num.to_string();
    let max = line_count.to_string().len();
    " ".repeat(max - str_num.len()) + &str_num + "|"
}

struct State {
    pos: (usize, usize),
    size: (u16, u16),
    content: String,
}
impl State {
    fn get_visible(&self) -> String {
        self.content
            .lines()
            .enumerate()
            .skip(self.pos.1)
            .take(self.size.1 as usize - 1)
            .map(|(index, line)| -> String {
                let line_indicator = line_indicator_format(index + 1, self.content.lines().count());
                let line_indicator_len = line_indicator.len();
                line_indicator
                    + line
                        .chars()
                        .skip(self.pos.0)
                        .take(self.size.0 as usize - line_indicator_len)
                        .collect::<String>()
                        .as_str()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn up(&mut self) -> bool {
        if self.pos.1 != 0 {
            self.pos.1 -= 1;
            return true;
        }
        false
    }

    fn down(&mut self) -> bool {
        if self.pos.1 != self.content.lines().count() - 1 {
            self.pos.1 += 1;
            return true;
        }
        false
    }

    fn left(&mut self) -> bool {
        if self.pos.0 != 0 {
            self.pos.0 -= 1;
            return true;
        }
        false
    }

    fn right(&mut self) -> bool {
        self.pos.0 += 1;
        true
    }

    fn pgup(&mut self) -> bool {
        if self.pos.1 >= self.size.1 as usize {
            self.pos.1 -= self.size.1 as usize - 1;
            return true;
        } else if self.pos.1 != 0 {
            self.pos.1 = 0;
            return true;
        }
        false
    }

    fn pgdown(&mut self) -> bool {
        let new = (self.pos.1 + self.size.1 as usize).min(self.content.lines().count()) - 1;
        if new != self.pos.1 {
            self.pos.1 = new;
            return true;
        }
        false
    }

    fn home(&mut self) -> bool {
        if self.pos.1 > 0 {
            self.pos.1 = 0;
            return true;
        }
        false
    }

    fn end(&mut self) -> bool {
        self.pos.1 = self.content.lines().count() - self.size.1 as usize + 1;
        true
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args.get(1).unwrap_or(&"input.txt".to_string()))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut out = stdout();
    out.queue(cursor::Hide)?;
    out.queue(terminal::EnterAlternateScreen)?;
    execute!(out, event::EnableMouseCapture)?;

    let mut state = State {
        pos: (0, 0),
        size: terminal::size()?,
        content,
    };

    execute!(out, cursor::MoveTo(0, 0))?;

    out.write_all(&state.get_visible().bytes().collect::<Vec<u8>>())?;
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
            out.write_all(&state.get_visible().bytes().collect::<Vec<u8>>())?;
            out.flush()?;
            enable_raw_mode()?;
        }
    }

    disable_raw_mode()?;
    out.queue(cursor::Show)?;
    out.queue(terminal::LeaveAlternateScreen)?;

    Ok(())
}
