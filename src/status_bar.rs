use crossterm::style::{Attribute, Color, ContentStyle, StyledContent, Stylize};

use crate::State;

#[derive(Clone, Debug)]
pub enum StatusBarLayoutItem {
    Text(String),
    Persentage,
    LineCount,
    CurrentLine,
    Title,
}

/// Layout for a [`StatusBar`] line.
#[derive(Clone, Debug)]
pub struct StatusBarLayout {
    /// Items that sticked to the left.
    pub left: Vec<StatusBarLayoutItem>,

    /// Items that sticked to the left.
    pub right: Vec<StatusBarLayoutItem>,
}

impl Default for StatusBarLayout {
    fn default() -> Self {
        use StatusBarLayoutItem::*;
        Self {
            left: vec![Title],
            right: vec![
                CurrentLine,
                Text("/".to_string()),
                LineCount,
                Text(" (".to_string()),
                Persentage,
                Text("%)".to_string()),
            ],
        }
    }
}
impl StatusBarLayout {
    /// Get left and right parts as string.
    fn get_parts(&self, state: &State) -> [String; 2] {
        let content_line_count = state.content.lines().count();
        [self.left.clone(), self.right.clone()].map(|part| {
            let mut output = String::new();
            for item in part {
                output += &match item {
                    StatusBarLayoutItem::Text(s) => s.clone(),
                    StatusBarLayoutItem::Persentage => {
                        format!(
                            "{:.0}",
                            (state.pos.1 as f32 / content_line_count as f32) * 100.0
                        )
                    }
                    StatusBarLayoutItem::LineCount => content_line_count.to_string(),
                    StatusBarLayoutItem::CurrentLine => (state.pos.1 + 1).to_string(),
                    StatusBarLayoutItem::Title => state.status_bar.title.clone(),
                };
            }
            output
        })
    }
}

#[derive(Clone, Debug)]
pub struct StatusBar {
    pub line_layouts: Vec<StatusBarLayout>,
    pub title: String,
    pub theme: ContentStyle,
}

impl StatusBar {
    /// Create a [`StatusBar`] with title.
    pub fn new(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }

    /// Create a [`StatusBar`] with title and theme.
    pub fn with_theme(title: String, theme: ContentStyle) -> Self {
        Self {
            title,
            theme,
            ..Default::default()
        }
    }

    /// Get status bar text to be printed on terminal.
    pub fn get_visible(&self, state: &State) -> StyledContent<String> {
        let bar = self
            .line_layouts
            .iter()
            .map(|layout| {
                let parts = layout.get_parts(state);
                let width = state.size.0 as usize;
                if parts[0].len() > width {
                    parts[0].chars().take(width).collect()
                } else if parts[0].len() + parts[1].len() > width {
                    parts[0].clone() + &" ".repeat(width - parts[0].len())
                } else {
                    parts[0].clone()
                        + &" ".repeat(width - parts[0].len() - parts[1].len())
                        + &parts[1]
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        self.theme.apply(bar)
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        let theme = ContentStyle::new()
            .with(Color::Black)
            .on(Color::White)
            .attribute(Attribute::Bold);
        Self {
            line_layouts: vec![StatusBarLayout::default()],
            title: "***".to_string(),
            theme,
        }
    }
}
