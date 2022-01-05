use crossterm::{
    event::KeyCode,
    style::{Attribute, Color, ContentStyle, Stylize},
    terminal,
};

use crate::{run, status_bar::StatusBar, StatusBarLayout, StatusBarLayoutItem};

pub fn line_indicator_format(line_num: String, line_count: usize) -> String {
    let max = line_count.to_string().len();
    " ".repeat(max - line_num.len()) + &line_num + "|"
}

#[derive(Clone)]
pub enum Command {
    Colon {
        cmd: String,
        desc: String,
        func: &'static dyn Fn(&mut State) -> bool,
    },
    Key {
        cmd: KeyCode,
        desc: String,
        func: &'static dyn Fn(&mut State) -> bool,
    },
}

pub struct CommandList(pub Vec<Command>);

impl Default for CommandList {
    fn default() -> Self {
        Self(vec![
            Command::Key {
                cmd: KeyCode::Char('q'),
                desc: "Quit".to_string(),
                func: &|state: &mut State| {
                    state.running = false;
                    false
                },
            },
            Command::Colon {
                cmd: "quit".to_string(),
                desc: "Quit".to_string(),
                func: &|state: &mut State| {
                    state.running = false;
                    false
                },
            },
            Command::Key {
                cmd: KeyCode::Up,
                desc: "Cursor up".to_string(),
                func: &|state: &mut State| state.up(),
            },
            Command::Key {
                cmd: KeyCode::Down,
                desc: "Cursor down".to_string(),
                func: &|state: &mut State| state.down(),
            },
            Command::Key {
                cmd: KeyCode::Left,
                desc: "Cursor left".to_string(),
                func: &|state: &mut State| state.left(),
            },
            Command::Key {
                cmd: KeyCode::Right,
                desc: "Cursor right".to_string(),
                func: &|state: &mut State| state.right(),
            },
            Command::Key {
                cmd: KeyCode::Home,
                desc: "Go to start".to_string(),
                func: &|state: &mut State| state.home(),
            },
            Command::Key {
                cmd: KeyCode::End,
                desc: "Go to end".to_string(),
                func: &|state: &mut State| state.end(),
            },
            Command::Key {
                cmd: KeyCode::PageUp,
                desc: "One page up".to_string(),
                func: &|state: &mut State| state.pgup(),
            },
            Command::Key {
                cmd: KeyCode::PageDown,
                desc: "One page down".to_string(),
                func: &|state: &mut State| state.pgdown(),
            },
            Command::Key {
                cmd: KeyCode::Char('h'),
                desc: "Toggles help text visiblity".to_string(),
                func: &|state: &mut State| {
                    let theme = ContentStyle::new()
                        .with(Color::Black)
                        .on(Color::White)
                        .attribute(Attribute::Bold);
                    let commands = CommandList(
                        state
                            .commands
                            .0
                            .clone()
                            .into_iter()
                            .filter(|command| matches!(command, Command::Key {cmd, ..} if *cmd != KeyCode::Char('h')))
                            .collect(),
                    );

                    let mut help = State {
                        pos: (0, 0),
                        size: state.size,
                        content: state.get_help_text(),
                        status_bar: StatusBar {
                            line_layouts: vec![StatusBarLayout {
                                left: vec![StatusBarLayoutItem::Text("Quit (q)".to_owned())],
                                right: vec![],
                            }],
                            title: "Help text".to_owned(),
                            theme,
                        },
                        commands,
                        running: true,
                    };
                    run(&mut help).unwrap();
                    true
                },
            },
        ])
    }
}

pub struct State {
    pub pos: (usize, usize),
    pub size: (u16, u16),
    pub content: String,
    pub status_bar: StatusBar,
    pub commands: CommandList,
    pub(crate) running: bool,
}

impl State {
    pub fn new(
        content: String,
        status_bar: StatusBar,
        commands: CommandList,
    ) -> std::io::Result<Self> {
        Ok(Self {
            pos: (0, 0),
            size: terminal::size()?,
            content,
            status_bar,
            commands,
            running: true,
        })
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn get_help_text(&self) -> String {
        if self.commands.0.is_empty() {
            return String::from("No commands");
        }
        let items = self.commands.0.iter().map(|command| match command {
            Command::Colon { cmd, desc, .. } => (":".to_owned() + cmd, desc.clone()),
            Command::Key { cmd, desc, .. } => (
                match *cmd {
                    KeyCode::Backspace => "Backspace".to_string(),
                    KeyCode::Enter => "Enter".to_string(),
                    KeyCode::Left => "Left".to_string(),
                    KeyCode::Right => "Right".to_string(),
                    KeyCode::Up => "Up".to_string(),
                    KeyCode::Down => "Down".to_string(),
                    KeyCode::Home => "Home".to_string(),
                    KeyCode::End => "End".to_string(),
                    KeyCode::PageUp => "PageUp".to_string(),
                    KeyCode::PageDown => "PageDown".to_string(),
                    KeyCode::Tab => "Tab".to_string(),
                    KeyCode::BackTab => "BackTab".to_string(),
                    KeyCode::Delete => "Delete".to_string(),
                    KeyCode::Insert => "Insert".to_string(),
                    KeyCode::F(n) => "F".to_string() + &n.to_string(),
                    KeyCode::Char(c) => c.to_string(),
                    KeyCode::Null => "Null".to_string(),
                    KeyCode::Esc => "Esc".to_string(),
                },
                desc.clone(),
            ),
        });
        let max_name_len = items.clone().map(|item| item.0.len()).max().unwrap();
        let padding = max_name_len + 2;

        items
            .map(|(name, desc)| {
                let name_len = name.len();
                name + &" ".repeat(padding - name_len)
                    + &desc
                        .lines()
                        .collect::<Vec<&str>>()
                        .join(("\n".to_string() + &" ".repeat(padding)).as_str())
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl State {
    pub fn get_visible(&self) -> String {
        self.content
            .lines()
            .enumerate()
            .skip(self.pos.1)
            .take(self.size.1 as usize - self.status_bar.line_layouts.len())
            .map(|(index, line)| -> String {
                let line_indicator =
                    line_indicator_format((index + 1).to_string(), self.content.lines().count());
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
}

impl State {
    pub fn up(&mut self) -> bool {
        if self.pos.1 != 0 {
            self.pos.1 -= 1;
            return true;
        }
        false
    }

    pub fn down(&mut self) -> bool {
        if self.pos.1 != self.content.lines().count() - 1 {
            self.pos.1 += 1;
            return true;
        }
        false
    }

    pub fn left(&mut self) -> bool {
        if self.pos.0 != 0 {
            self.pos.0 -= 1;
            return true;
        }
        false
    }

    pub fn right(&mut self) -> bool {
        self.pos.0 += 1;
        true
    }

    pub fn pgup(&mut self) -> bool {
        if self.pos.1 >= self.size.1 as usize {
            self.pos.1 -= self.size.1 as usize - 1;
            return true;
        } else if self.pos.1 != 0 {
            self.pos.1 = 0;
            return true;
        }
        false
    }

    pub fn pgdown(&mut self) -> bool {
        let new = (self.pos.1 + self.size.1 as usize).min(self.content.lines().count()) - 1;
        if new != self.pos.1 {
            self.pos.1 = new;
            return true;
        }
        false
    }

    pub fn home(&mut self) -> bool {
        if self.pos.1 > 0 {
            self.pos.1 = 0;
            return true;
        }
        false
    }

    pub fn end(&mut self) -> bool {
        self.pos.1 = self.content.lines().count() - self.size.1 as usize + 1;
        true
    }
}

impl State {
    pub fn match_key_event(&mut self, code: KeyCode) -> bool {
        for command in self.commands.0.iter() {
            if let Command::Key { cmd, func, .. } = *command {
                if code == cmd {
                    return func(self);
                }
            }
        }
        false
    }
}
