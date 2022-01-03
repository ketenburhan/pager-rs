use crate::status_bar::StatusBar;

pub fn line_indicator_format(line_num: String, line_count: usize) -> String {
    let max = line_count.to_string().len();
    " ".repeat(max - line_num.len()) + &line_num + "|"
}

pub struct State {
    pub pos: (usize, usize),
    pub size: (u16, u16),
    pub content: String,
    pub status_bar: StatusBar,
}
impl State {
    pub fn get_visible(&self) -> String {
        self.content
            .lines()
            .enumerate()
            .skip(self.pos.1)
            .take(self.size.1 as usize - self.status_bar.line_count as usize)
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
