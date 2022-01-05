use crossterm::style::{Attribute, Color, ContentStyle, Stylize};
use pager_rs::{CommandList, State, StatusBar};
fn main() -> std::io::Result<()> {
    let content = r#"fn main() {
    println!("Hello World!");
}"#
    .to_string();

    let theme = ContentStyle::new()
        .with(Color::White)
        .on(Color::Red)
        .attribute(Attribute::Italic);
    let status_bar = StatusBar::with_theme(
        "Hello World program in rust with colored status bar".to_string(),
        theme,
    );

    let mut state = State::new(content, status_bar, CommandList::default())?;

    pager_rs::init()?;

    pager_rs::run(&mut state)?;

    pager_rs::finish()?;

    Ok(())
}
