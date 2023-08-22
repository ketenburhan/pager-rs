use crossterm::{
    event::KeyCode,
    style::{Attribute, Color, ContentStyle, Stylize},
    terminal,
};

use crate::{run, status_bar::StatusBar, StatusBarLayout, StatusBarLayoutItem};

/// Type of [`Command`]
#[derive(Clone, PartialEq)]
pub enum CommandType {
    /// Waits for `:` key and then the command input, until Enter is pressed.
    Colon(String),
    /// Waits for key input.
    Key(KeyCode),
}

#[derive(Clone)]
pub struct Command {
    /// When any of the values matched with input from user, command will be executed.
    pub cmd: Vec<CommandType>,
    /// Description of the command, can be seen in help text.
    pub desc: String,
    /// The function that runs when command executed.
    pub func: &'static dyn Fn(&mut State) -> bool,
}

/// Container of list of commands.
pub struct CommandList(pub Vec<Command>);

impl From<CommandList> for Vec<Command> {
    fn from(val: CommandList) -> Self {
        val.0
    }
}

impl CommandList {
    /// Combine [`CommandList`]'s into one.
    pub fn combine<T>(list: Vec<T>) -> Self
    where
        T: Into<Vec<Command>>,
    {
        let mut v = vec![];
        for item in list {
            v.append(&mut item.into());
        }
        Self(v)
    }

    /// Default 'quit' command
    pub fn quit() -> Self {
        use CommandType::*;
        Self(vec![Command {
            cmd: vec![Key(KeyCode::Char('q')), Colon("quit".to_string())],
            desc: "Quit".to_string(),
            func: &|state: &mut State| {
                state.quit();
                false
            },
        }])
    }

    /// Default bundle of 'navigation' commands.
    ///
    /// Includes: `Arrow`, `Home/End`, `PageUp/PageDown` keys
    pub fn navigation() -> Self {
        use CommandType::*;
        Self(vec![
            Command {
                cmd: vec![Key(KeyCode::Up)],
                desc: "Cursor up".to_string(),
                func: &|state: &mut State| state.up(),
            },
            Command {
                cmd: vec![Key(KeyCode::Down)],
                desc: "Cursor down".to_string(),
                func: &|state: &mut State| state.down(),
            },
            Command {
                cmd: vec![Key(KeyCode::Left)],
                desc: "Cursor left".to_string(),
                func: &|state: &mut State| state.left(),
            },
            Command {
                cmd: vec![Key(KeyCode::Right)],
                desc: "Cursor right".to_string(),
                func: &|state: &mut State| state.right(),
            },
            Command {
                cmd: vec![Key(KeyCode::Home), Key(KeyCode::Char('g'))],
                desc: "Go to start".to_string(),
                func: &|state: &mut State| state.home(),
            },
            Command {
                cmd: vec![Key(KeyCode::End), Key(KeyCode::Char('G'))],
                desc: "Go to end".to_string(),
                func: &|state: &mut State| state.end(),
            },
            Command {
                cmd: vec![Key(KeyCode::PageUp)],
                desc: "One page up".to_string(),
                func: &|state: &mut State| state.pgup(),
            },
            Command {
                cmd: vec![Key(KeyCode::PageDown)],
                desc: "One page down".to_string(),
                func: &|state: &mut State| state.pgdown(),
            },
        ])
    }

    /// Default 'help' command
    pub fn help() -> Self {
        use CommandType::*;
        Self(vec![Command {
            cmd: vec![Key(KeyCode::Char('h')), Colon("help".to_string())],
            desc: "Toggles help text visiblity".to_string(),
            func: &|state: &mut State| {
                let theme = ContentStyle::new()
                    .with(Color::Black)
                    .on(Color::White)
                    .attribute(Attribute::Bold);
                let commands =
                    CommandList::combine(vec![CommandList::quit(), CommandList::navigation()]);

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
                    show_line_numbers: false,
                };
                run(&mut help).unwrap();
                true
            },
        }])
    }

    /// Default 'toggle line numbers' command
    pub fn toggle_line_numbers() -> Self {
        use CommandType::*;
        Self(vec![Command {
            cmd: vec![Key(KeyCode::Char('l'))],
            desc: "Show/Hide line numbers".to_string(),
            func: &|state: &mut State| {
                state.show_line_numbers = !state.show_line_numbers;
                true
            },
        }])
    }
}

impl Default for CommandList {
    fn default() -> Self {
        Self::combine(vec![
            Self::quit(),
            Self::navigation(),
            Self::help(),
            Self::toggle_line_numbers(),
        ])
    }
}

pub struct State {
    /// Cursor position in content.
    ///
    /// `(x, y)`
    pub pos: (usize, usize),

    /// Size of terminal screen.
    ///
    /// `(width, height)`
    pub size: (u16, u16),

    /// Content to show.
    pub content: String,

    /// status bar at the bottom.
    pub status_bar: StatusBar,

    pub commands: CommandList,

    pub(crate) running: bool,

    pub show_line_numbers: bool,
}

impl State {
    /// Create new [`State`]
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
            show_line_numbers: true,
        })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Terminate [`State`]
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Default help text formatter
    pub fn get_help_text(&self) -> String {
        if self.commands.0.is_empty() {
            return String::from("No commands");
        }
        let items = self.commands.0.iter().map(|command| {
            let name = command
                .cmd
                .iter()
                .map(|cmd_type| match cmd_type {
                    CommandType::Key(code) => match *code {
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
                        KeyCode::F(n) => format!("F{}", n),
                        KeyCode::Char(c) => c.to_string(),
                        KeyCode::Null => "Null".to_string(),
                        KeyCode::Esc => "Esc".to_string(),
                        KeyCode::CapsLock => "CapsLock".to_string(),
                        KeyCode::NumLock => "NumLock".to_string(),
                        KeyCode::ScrollLock => "ScrollLock".to_string(),
                        KeyCode::PrintScreen => "PrintScreen".to_string(),
                        KeyCode::Pause => "Pause".to_string(),
                        KeyCode::Menu => "Menu".to_string(),
                        KeyCode::KeypadBegin => "KeypadBegin".to_string(),
                        KeyCode::Media(_) => "MediaKey".to_string(),
                        KeyCode::Modifier(_) => "ModifierKey".to_string(),
                    },
                    CommandType::Colon(s) => format!(":{}", s),
                })
                .collect::<Vec<String>>()
                .join(", ");
            (name, command.desc.clone())
        });
        let max_name_len = items.clone().map(|item| item.0.len()).max().unwrap();
        let padding = max_name_len + 2;

        items
            .map(|(name, desc)| {
                let name_len = name.len();
                format!(
                    "{}{gap}{}",
                    name,
                    desc.lines()
                        .collect::<Vec<&str>>()
                        .join(format!("\n{}", " ".repeat(padding)).as_str()),
                    gap = " ".repeat(padding - name_len),
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl State {
    /// Get text to be printed on terminal except for the [`StatusBar`].
    pub fn get_visible(&self) -> String {
        let max_line_number_width = self.content.lines().count().to_string().len();
        self.content
            .lines()
            .enumerate()
            .skip(self.pos.1)
            .take(self.size.1 as usize - self.status_bar.line_layouts.len())
            .map(|(index, line)| -> String {
                let line_indicator = if self.show_line_numbers {
                    format!(
                        "{:line_count$}│",
                        index + 1,
                        line_count = max_line_number_width
                    )
                } else {
                    String::new()
                };
                let line_indicator_len = line_indicator.chars().count();
                format!(
                    "{line_indicator}{visible_content_line}",
                    line_indicator = line_indicator,
                    visible_content_line = line
                        .chars()
                        .skip(self.pos.0)
                        .take(self.size.0 as usize - line_indicator_len)
                        .collect::<String>()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl State {
    /// Move cursor up.
    pub fn up(&mut self) -> bool {
        if self.pos.1 != 0 {
            self.pos.1 -= 1;
            return true;
        }
        false
    }

    /// Move cursor down.
    pub fn down(&mut self) -> bool {
        if self.pos.1 != self.content.lines().count() - 1 {
            self.pos.1 += 1;
            return true;
        }
        false
    }

    /// Move cursor left.
    pub fn left(&mut self) -> bool {
        let amount = self.size.0 as usize / 2;
        if self.pos.0 >= amount {
            self.pos.0 -= amount;
            return true;
        } else if self.pos.0 != 0 {
            self.pos.0 = 0;
            return true;
        }
        false
    }

    /// Move cursor right.
    pub fn right(&mut self) -> bool {
        let amount = self.size.0 as usize / 2;
        self.pos.0 += amount;
        true
    }

    /// Move cursor one page up.
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

    /// Move cursor one page down.
    pub fn pgdown(&mut self) -> bool {
        let new = (self.pos.1 + self.size.1 as usize).min(self.content.lines().count()) - 1;
        if new != self.pos.1 {
            self.pos.1 = new;
            return true;
        }
        false
    }

    /// Move cursor to the start.
    pub fn home(&mut self) -> bool {
        if self.pos.1 > 0 {
            self.pos.1 = 0;
            return true;
        }
        false
    }

    /// Move cursor to the end.
    pub fn end(&mut self) -> bool {
        let line_count = self.content.lines().count();
        self.pos.1 = if line_count > self.size.1 as usize {
            line_count - self.size.1 as usize + 1
        } else {
            0
        };
        true
    }
}

impl State {
    /// Find and execute command matching with pressed key.
    pub fn match_key_event(&mut self, code: KeyCode) -> bool {
        let mut commands = self.commands.0.clone();
        let found = commands
            .iter_mut()
            .find(|command| command.cmd.contains(&CommandType::Key(code)));
        if let Some(Command { func, .. }) = found {
            return func(self);
        }
        false
    }
}
