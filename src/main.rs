use std::{env, fs::File, io::Read, time::Duration};

use console::{Key, Term};
use std::thread;

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
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args.get(1).unwrap_or(&"input.txt".to_string()))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let term = Term::stdout();
    term.hide_cursor()?;
    term.clear_screen()?;

    let size = term.size();
    let mut state = State {
        pos: (0, 0),
        size: (size.1, size.0),
        content,
    };

    term.write_line(&state.get_visible())?;

    loop {
        let flash = match term.read_key() {
            Ok(Key::ArrowUp) => state.up(),
            Ok(Key::ArrowDown) => state.down(),
            Ok(Key::ArrowLeft) => state.left(),
            Ok(Key::ArrowRight) => state.right(),
            Ok(Key::PageUp) => state.pgup(),
            Ok(Key::PageDown) => state.pgdown(),
            Ok(Key::Escape) | Ok(Key::Char('q')) | Ok(Key::Char('Q')) | Err(_) => break,
            _ => false,
        };
        if flash {
            term.clear_screen()?;
            term.write_line(&state.get_visible())?;
        }
    }
    thread::sleep(Duration::from_millis(200));

    term.clear_line()?;
    term.show_cursor()?;
    Ok(())
}
